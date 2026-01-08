# Blue Archive Decoder

A web application for decoding Blue Archive's database files, built with Rust, Leptos, and WebAssembly.

## Prerequisites

* **Rust** (latest stable)
* **wasm-pack**: `cargo install wasm-pack`

## Building

Build the project for the web target:

```bash
wasm-pack build --target web
```

## Running

After building, the artifacts will be in the `pkg/` directory. Because this project uses ES modules, you must serve the directory using a local web server rather than opening `index.html` directly (to avoid CORS errors).

Example using Python:

```bash
python3 -m http.server
# Open http://localhost:8000 in your browser
```

## License

Licensed under the **AGPL-3.0-only** license.
