# WASM Christmas Tree

Just a bit playing around with Rust, WASM and WebGL.
A follow-up on [Rusted Christmas tree](https://github.com/jacekbilski/rusted-christmas-tree).
For more up-to-date version visit [Vulkan Christmas Tree](https://github.com/jacekbilski/vulkan-christmas-tree).

### Building

Make sure to install `wasm-pack` using `cargo install wasm-pack` before running `./build.sh`.

### Running

As the app is using JavaScript modules, it needs to be served by an actual HTTP server.
Simplest way is to use [Serve These Things Please](https://crates.io/crates/https), so run `cargo install https`.
Now start the server in this project directory with `http` and go to [http://locahost:8000/](http://localhost:8000/).
