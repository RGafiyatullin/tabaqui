Tabaqui — a small and simple HTTP-proxy for debug purposes.

# Usage

## The Proxy
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

## The CLI
```
$ (export BASE_URI='http://127.0.0.1:8080'; for id in $(target/debug/tabaqui-query ids) ; do target/debug/tabaqui-query get $id; echo; done)
ID: 0
RQ: at 2019-10-11 10:30:24
	GET /okay-google?yep=1 HTTP/1.1
	connection: keep-alive
	accept-encoding: gzip, deflate, br
	user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.90 Safari/537.36
	host: localhost:8080
	upgrade-insecure-requests: 1
	accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3
	sec-fetch-mode: navigate
	accept-language: en-GB,en-US;q=0.9,en;q=0.8,ru;q=0.7
	sec-fetch-site: none


RS: at 2019-10-11 10:30:24
	HTTP/1.1 404 Not Found
	date: Fri, 11 Oct 2019 10:30:11 GMT
	content-length: 1572
	referrer-policy: no-referrer
	content-type: text/html; charset=UTF-8

<!DOCTYPE html>
<html lang=en>
  <meta charset=utf-8>
  <meta name=viewport content="initial-scale=1, minimum-scale=1, width=device-width">
  <title>Error 404 (Not Found)!!1</title>
  <style>
    *{margin:0;padding:0}html,code{font:15px/22px arial,sans-serif}html{background:#fff;color:#222;padding:15px}body{margin:7% auto 0;max-width:390px;min-height:180px;padding:30px 0 15px}* > body{background:url(//www.google.com/images/errors/robot.png) 100% 5px no-repeat;padding-right:205px}p{margin:11px 0 22px;overflow:hidden}ins{color:#777;text-decoration:none}a img{border:0}@media screen and (max-width:772px){body{background:none;margin-top:0;max-width:none;padding-right:0}}#logo{background:url(//www.google.com/images/branding/googlelogo/1x/googlelogo_color_150x54dp.png) no-repeat;margin-left:-5px}@media only screen and (min-resolution:192dpi){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat 0% 0%/100% 100%;-moz-border-image:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) 0}}@media only screen and (-webkit-min-device-pixel-ratio:2){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat;-webkit-background-size:100% 100%}}#logo{display:inline-block;height:54px;width:150px}
  </style>
  <a href=//www.google.com/><span id=logo aria-label=Google></span></a>
  <p><b>404.</b> <ins>That’s an error.</ins>
  <p>The requested URL <code>/okay-google</code> was not found on this server.  <ins>That’s all we know.</ins>


ID: 1
RQ: at 2019-10-11 10:30:24
	GET /hey-you HTTP/1.1
	sec-fetch-user: ?1
	accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3
	connection: keep-alive
	upgrade-insecure-requests: 1
	sec-fetch-site: none
	accept-encoding: gzip, deflate, br
	accept-language: en-GB,en-US;q=0.9,en;q=0.8,ru;q=0.7
	user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.90 Safari/537.36
	sec-fetch-mode: navigate
	host: localhost:8080


RS: at 2019-10-11 10:30:24
	HTTP/1.1 404 Not Found
	content-length: 1568
	referrer-policy: no-referrer
	content-type: text/html; charset=UTF-8
	date: Fri, 11 Oct 2019 10:30:15 GMT

<!DOCTYPE html>
<html lang=en>
  <meta charset=utf-8>
  <meta name=viewport content="initial-scale=1, minimum-scale=1, width=device-width">
  <title>Error 404 (Not Found)!!1</title>
  <style>
    *{margin:0;padding:0}html,code{font:15px/22px arial,sans-serif}html{background:#fff;color:#222;padding:15px}body{margin:7% auto 0;max-width:390px;min-height:180px;padding:30px 0 15px}* > body{background:url(//www.google.com/images/errors/robot.png) 100% 5px no-repeat;padding-right:205px}p{margin:11px 0 22px;overflow:hidden}ins{color:#777;text-decoration:none}a img{border:0}@media screen and (max-width:772px){body{background:none;margin-top:0;max-width:none;padding-right:0}}#logo{background:url(//www.google.com/images/branding/googlelogo/1x/googlelogo_color_150x54dp.png) no-repeat;margin-left:-5px}@media only screen and (min-resolution:192dpi){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat 0% 0%/100% 100%;-moz-border-image:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) 0}}@media only screen and (-webkit-min-device-pixel-ratio:2){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat;-webkit-background-size:100% 100%}}#logo{display:inline-block;height:54px;width:150px}
  </style>
  <a href=//www.google.com/><span id=logo aria-label=Google></span></a>
  <p><b>404.</b> <ins>That’s an error.</ins>
  <p>The requested URL <code>/hey-you</code> was not found on this server.  <ins>That’s all we know.</ins>


ID: 2
RQ: at 2019-10-11 10:30:24
	GET / HTTP/1.1
	accept-encoding: gzip, deflate, br
	connection: keep-alive
	user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.90 Safari/537.36
	upgrade-insecure-requests: 1
	host: localhost:8080
	sec-fetch-mode: navigate
	sec-fetch-site: none
	accept-language: en-GB,en-US;q=0.9,en;q=0.8,ru;q=0.7
	sec-fetch-user: ?1
	accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3


RS: at 2019-10-11 10:30:24
	HTTP/1.1 404 Not Found
	content-length: 1561
	content-type: text/html; charset=UTF-8
	date: Fri, 11 Oct 2019 10:30:19 GMT
	referrer-policy: no-referrer
    
<!DOCTYPE html>
<html lang=en>
  <meta charset=utf-8>
  <meta name=viewport content="initial-scale=1, minimum-scale=1, width=device-width">
  <title>Error 404 (Not Found)!!1</title>
  <style>
    *{margin:0;padding:0}html,code{font:15px/22px arial,sans-serif}html{background:#fff;color:#222;padding:15px}body{margin:7% auto 0;max-width:390px;min-height:180px;padding:30px 0 15px}* > body{background:url(//www.google.com/images/errors/robot.png) 100% 5px no-repeat;padding-right:205px}p{margin:11px 0 22px;overflow:hidden}ins{color:#777;text-decoration:none}a img{border:0}@media screen and (max-width:772px){body{background:none;margin-top:0;max-width:none;padding-right:0}}#logo{background:url(//www.google.com/images/branding/googlelogo/1x/googlelogo_color_150x54dp.png) no-repeat;margin-left:-5px}@media only screen and (min-resolution:192dpi){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat 0% 0%/100% 100%;-moz-border-image:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) 0}}@media only screen and (-webkit-min-device-pixel-ratio:2){#logo{background:url(//www.google.com/images/branding/googlelogo/2x/googlelogo_color_150x54dp.png) no-repeat;-webkit-background-size:100% 100%}}#logo{display:inline-block;height:54px;width:150px}
  </style>
  <a href=//www.google.com/><span id=logo aria-label=Google></span></a>
  <p><b>404.</b> <ins>That’s an error.</ins>
  <p>The requested URL <code>/</code> was not found on this server.  <ins>That’s all we know.</ins>
```
