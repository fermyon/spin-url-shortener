# A simple URL shortener built with Spin

This is a simple URL shortener built with Spin, running as a WebAssembly module.

The only configuration needed is the source and destination to redirect to:

```toml
[[route]]
source = "/spin"
destination = "https://github.com/fermyon/spin"

[[route]]
source = "/hype"
destination = "https://www.fermyon.com/blog/how-to-think-about-wasm"
```

Running can be done from the root of the repository with Spin:

```bash
$ spin up --file spin.toml
```

Then, sending requests to routes configured in `routes.toml` returns the response
pointing to the configured destination:

```bash
$ curl -i localhost:3000/spin
HTTP/1.1 308 Permanent Redirect
location: https://github.com/fermyon/spin
content-length: 0
date: Tue, 22 Mar 2022 03:02:00 GMT

$ curl -i localhost:3000/hype
HTTP/1.1 308 Permanent Redirect
location: https://www.fermyon.com/blog/how-to-think-about-wasm
content-length: 0
```
