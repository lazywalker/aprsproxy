# AprsProxy

I was thinking how to monitor APRS statistic from my iGates (from me or friends), one proper way is to run a APRS proxy server between iGates and APRS-IS server. But I just could not find a software to do this so I decided to write my own.

This APRS proxy server would received packets from devices, logged it files, generate statistic or send/forward specific aprs packets to other application, replace the text to meet my requirement and resend packets to native APRS-IS server.

## Features

* Text replacing with multiple keywords
* Build-in forwarder
* Simple, no config file with only one command
* Handle multi client connections with highly efficiency
* You can use aprs-is via domain name
* Daily log to files

## Command

``` bash
$ ./aprsproxy -h
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
    -q, --quiet      Quiet mode, no output to stdout
    -V, --version    Prints version information
    -v, --verbose    Verbose mode (-v, -vv, -vvv, etc.)

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
$ ./aprsproxy --replace=SO23 --with=T3ET -f -v
[2021-07-27T02:33:45Z INFO  aprsproxy] Starting up...
[2021-07-27T02:33:45Z INFO  aprsproxy::dns] Resolving ip address...
[2021-07-27T02:33:45Z INFO  aprsproxy::relay] Listening on: 0.0.0.0:8074
[2021-07-27T02:33:45Z INFO  aprsproxy::relay] Proxying to: 43.245.198.229:14580
[2021-07-27T02:33:45Z INFO  aprsproxy::relay] A new connection 113.118.74.21:47851 is coming!
[2021-07-27T02:33:46Z INFO  aprsproxy::relay] user B**** pass ****** vers APRS-51WG3-8K-20190610
[2021-07-27T02:34:06Z INFO  aprsproxy::relay] BI7KCD-9>R2UYX6,BA7NQ-10*,WIDE1*,BH7KCJ-3*,WIDE2*:`)XQm4>/`"42}_%
[2021-07-27T02:34:53Z INFO  aprsproxy::relay] BI7KCD-9>R3PPR7,BA7NQ-10*,WIDE1*,BH7KCJ-3*,WIDE2*:`)XRmpC>/`"4#}_%
[2021-07-27T02:35:07Z INFO  aprsproxy::relay] VR2ZVR>RR2VT1,BH7KCJ-3*,WIDE1*:`*_LnplK\]"4s}Hello from VR2ZVR...Stay Safe !!=
```
