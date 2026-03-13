---
title: HTTP Headers
---

> HTTP headers let the client and the server pass additional information with a
> message in a request or response.

[HTTP Headers](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers)

### Question: Where exactly is HTTP headers stored?

HTTP headers are not stored in one permanent place; they live briefly in memory
at each hop (browser, proxies, server) as part of the HTTP message being sent or
received.

#### Client side:

The browser builds an HTTP request message in its process memeory, which is
written into a TCP/IP packet buffer sent over the network (nothing is saved on
disk so to speak). The browser may keep some header in internal data structures
for cahcing or cookies.

#### Server side:

The web server process (apache2, nginx, node.js) receives the TCP stream and
parses out the HTTP headers for your web app to use. Default server headers
cofiguration (apache config, nginx config, .htaccess) do live on disk. Each
actual header that goes to the client is ephermeral and generated on the fly in
memory for each response.

#### Proxy and Cache:

CDNs, reverse proxies, or browser's cache store HTTP messages (both headers and
body) in memory or disk for reuse.

# HTTP Security Headers

[HTTP Security Headers: A complete guide to HTTP headers](https://www.darkrelay.com/post/http-security-headers)

[Analyze your headers](https://securityheaders.com/)

- Access-Control-Allow-Origin Security Header

- Content-Type Header

- Content-Security-Policy (CSP) Security Header

- Cross-Origin-Embedder-Policy Security Header

- Cross-Origin-Resource-Policy Security Header

- Cross-Origin-Opener-Policy Security Header

- Set-Cookie Header

- Strict-Transport-Security (HSTS) Security Header

- Referrer-Policy Header

- X-Content-Type-Options Security Header

- X-Frame-Options Security Header

- X-XSS-Protection Security Header

- X-Permitted-Cross-Domain-Policies Security Header

- Cache-Control Header

- X-Powered-By Header

- Public-Key-Pins(HPKP) header
