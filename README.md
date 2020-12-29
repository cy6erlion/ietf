``` text
█▀▄ █▀ ▄▀▀
█▀▄ █▀ ▀▄▄
```
rfc 0.1.0
A program to read RFCs in the terminal.

USAGE:

	rfc [OPTIONS] [SUBCOMMAND]
	
	FLAGS:
		-h, --help       Prints help information
		-V, --version    Prints version information
			
	OPTIONS:
		-n, --number <serial>    RFC Serial Number

	SUBCOMMANDS:
		help      Prints this message or the help of the given subcommand(s)
		update    Update RFC Index
						
## Features
* RFC index browser
* Read By RFC number
* Local Storage in (`~/rfc/` on unix systems and `C:\Users\{NAME}` on
  windows)
* Pager 
		
## Guide

### Running	
To run simply type the following command in the shell to start the RFC
browser:

``` bash
$ rfc
```

### Read RFC by number
``` bash
$ rfc -n 1
```

### Update
To update the local RFC index, use the following command:

``` bash
$ rfc update
```
