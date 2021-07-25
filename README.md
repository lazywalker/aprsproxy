# AprsProxy 0.1.0
``` bash
   ___                ___
  / _ | ___  _______ / _ \_______ __ ____ __
 / __ |/ _ \/ __(_-</ ___/ __/ _ \\ \ / // /
/_/ |_/ .__/_/ /___/_/  /_/  \___/_\_\\_, /
     /_/                             /___/
                A simply APRS-IS proxy tool.

USAGE:
    aprsproxy [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --remote <host-addr>        The remote address and port to connect to [default: china.aprs2.net:14580]
    -l, --local <local-addr>        The local address and port to listen on [default: 0.0.0.0:14580]
        --replace <replace-from>    The text to be replaced
        --with <replace-with>       The text to replace with
        
```