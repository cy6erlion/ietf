``` console
██▄██ ▄▄█▄ ▄█ ▄▄
██ ▄█ ▄▄██ ██ ▄█
█▄▄▄█▄▄▄██▄██▄██
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀

ietf 0.2.0
A program to read RFCs in the terminal.

USAGE:
    ietf [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --number <serial>    RFC Serial Number
    -r, --remove <serial>    RFC Serial Number

SUBCOMMANDS:
    clean     Remove the rfc directory
    help      Prints this message or the help of the given subcommand(s)
    update    Update RFC Index					
```						

## Features
* RFC index browser
* Read By RFC number
* Local Storage in (`~/rfc/` on unix systems and `C:\Users\{NAME}` on
  windows)
* Pager 
		
## Guide

### Installing
You can install it with the Rust package manager 
[Cargo](https://github.com/rust-lang/cargo) like this:

``` bash
 $ cargo install ietf
```

### View RFC index

``` bash
$ ietf
```

### Read RFC by number
``` bash
$ ietf -n 1
```

### Remove an RFC
``` bash
$ ietf -r 1
```

### Remove the RFC storage directory:
``` bash
$ ietf clean
```

### Update/Download RFC index

``` bash
$ ietf update
```
