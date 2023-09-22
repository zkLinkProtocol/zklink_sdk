# Web example 


You can build the example locally with:

```
$ wasm-pack build --target web --out-name=zklink-sdk-web --out-dir=web-dist
```

Then serve this directory in your favourite webserver and navigate to `host:port`
to open the index.html in your browser:

```
# static server from https://crates.io/crates/https
http

# or use python
python2 -m SimpleHTTPServer
python3 -m http.server
```
