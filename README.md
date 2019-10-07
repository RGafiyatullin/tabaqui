Tabaqui — a small and simple HTTP-proxy for debug purposes.

# Usage

```
$ ./target/debug/tabaqui --help
tabaqui

USAGE:
    tabaqui [FLAGS] [OPTIONS] -b <BACKEND-URI>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v               set verbosity

OPTIONS:
    -b <BACKEND-URI>         backend root-uri
    -a <BIND-ADDR>           address to bind
    -p <BIND-PORT>           port to bind
    -s <STORAGE-SIZE>        storage size (entries)
```

Tabaqui will proxy all the inbound requests to the specified backend.

The last `<STORAGE-SIZE>` request-response pairs will be available via the `/__mgmt` HTTP-endpoint:

```json
$ http get http://localhost:8080/__mgmt
HTTP/1.1 200 OK
content-length: 7598
content-type: application/json
date: Thu, 10 Oct 2019 21:12:30 GMT

{
    "cap": 100,
    "ids": [
        0,
        1
    ],
    "requests": {
        "0": {
            "at": {
                "nanos_since_epoch": 693706000,
                "secs_since_epoch": 1570741923
            },
            "body": [],
            "headers": {
                "accept": "*/*",
                "accept-encoding": "gzip, deflate",
                "connection": "keep-alive",
                "host": "localhost:8080",
                "user-agent": "HTTPie/1.0.2"
            },
            "method": "GET",
            "path": "/",
            "query": null
        },
        "1": {
            "at": {
                "nanos_since_epoch": 440255000,
                "secs_since_epoch": 1570741940
            },
            "body": [],
            "headers": {
                "accept": "*/*",
                "accept-encoding": "gzip, deflate",
                "connection": "keep-alive",
                "host": "google.com",
                "user-agent": "HTTPie/1.0.2"
            },
            "method": "GET",
            "path": "/",
            "query": null
        }
    },
    "responses": {
        "0": {
            "at": {
                "nanos_since_epoch": 918559000,
                "secs_since_epoch": 1570741923
            },
            "body": [ ... ],
            "headers": {
                "content-length": "1561",
                "content-type": "text/html; charset=UTF-8",
                "date": "Thu, 10 Oct 2019 21:12:03 GMT",
                "referrer-policy": "no-referrer"
            },
            "status": 404
        },
        "1": {
            "at": {
                "nanos_since_epoch": 652025000,
                "secs_since_epoch": 1570741940
            },
            "body": [ ... ],
            "headers": {
                "cache-control": "public, max-age=2592000",
                "content-length": "219",
                "content-type": "text/html; charset=UTF-8",
                "date": "Thu, 10 Oct 2019 21:12:20 GMT",
                "expires": "Sat, 09 Nov 2019 21:12:20 GMT",
                "location": "http://www.google.com/",
                "server": "gws",
                "x-frame-options": "SAMEORIGIN",
                "x-xss-protection": "0"
            },
            "status": 301
        }
    },
    "seq": 2
}

```

