# AprsProxy 0.2.1
``` bash
   ___                ___                   
  / _ | ___  _______ / _ \_______ __ ____ __
 / __ |/ _ \/ __(_-</ ___/ __/ _ \\ \ / // /
/_/ |_/ .__/_/ /___/_/  /_/  \___/_\_\\_, / 
     /_/                             /___/  
                A simply APRS-IS proxy tool.

USAGE:
    aprsproxy [FLAGS] [OPTIONS]

FLAGS:
    -f, --filelog    Enable file logging
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --to <forward-to>              Forward the matched APRS packets to Send-only APRS-IS service with http protocol
                                       [default: china.aprs2.net:8080]
        --forward <forward-with>...    Forward APRS packets start with the line prefix
    -l, --local <local-addr>           The local address and port to listen on [default: 0.0.0.0:14580]
    -r, --remote <remote-addr>         The remote address and port to connect to [default: china.aprs2.net:14580]
        --replace <replace-from>...    The text to be replaced, can be multiple values
        --with <replace-with>...       The text to replace with, must be the same length of replace-from
```


## Example
```log
$ RUST_LOG=info ./aprsproxy --replace=BD7MQB-13 --with=SZWX-13 -f
[2021-07-27T02:33:45Z INFO  aprsproxy] Starting up...
[2021-07-27T02:33:45Z INFO  aprsproxy::dns] Resolving ip address...
[2021-07-27T02:33:45Z INFO  aprsproxy::relay] Listening on: 0.0.0.0:8074
[2021-07-27T02:33:45Z INFO  aprsproxy::relay] Proxying to: 43.245.198.229:14580
[2021-07-27T02:33:45Z INFO  aprsproxy::relay] A new connection 113.118.74.21:47851 is coming!
[2021-07-27T02:33:46Z INFO  aprsproxy::relay] user BD7MQB-10 pass ****** vers APRS-51WG3-8K-20190610
[2021-07-27T02:34:06Z INFO  aprsproxy::relay] BI7KCD-9>R2UYX6,BA7NQ-10*,WIDE1*,BH7KCJ-3*,WIDE2*:`)XQm4>/`"42}_%
[2021-07-27T02:34:53Z INFO  aprsproxy::relay] BI7KCD-9>R3PPR7,BA7NQ-10*,WIDE1*,BH7KCJ-3*,WIDE2*:`)XRmpC>/`"4#}_%
[2021-07-27T02:35:07Z INFO  aprsproxy::relay] VR2ZVR>RR2VT1,BH7KCJ-3*,WIDE1*:`*_LnplK\]"4s}Hello from VR2ZVR...Stay Safe !!=
```