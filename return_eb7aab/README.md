
compile to WASM
---------------

with a proper main on the rust side:  
`rustc --target=wasm32-unknown-emscripten src/lib.rs -o some/target/directory/call_eb7aab.html`

WASM/JS module (call rust functions from JS):  
`rustc --target=wasm32-unknown-emscripten src/lib.rs -o ../call_eb7aab_wasm_b/return_eb7aab.js`

Thanks to [The Path to Rust on the Web](http://asquera.de/blog/2017-04-10/the-path-to-rust-on-the-web/) this worked almost instantly : )
