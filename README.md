<div align="center">

  <h1>WASM Game of Life</h1>

  <strong>Conway's Game of Life implemented in Rust and WebAssembly.</strong>

</div>

## About

This is a personal project implementing Conway's Game of Life in Rust, compiled into WebAssembly using `wasm-pack`. It is designed to be integrated into a React frontend (via Vite) for high-performance state calculations.

## 🚴 Usage

### 🐑 Clone this repo and install prerequisites

```bash
git clone https://github.com/LikhithST/wasm-game-of-life.git
cd wasm-game-of-life
cargo install cargo-generate
cargo install wasm-pack
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you



## Integration of WASM with Vite

1.  **Include `pkg` folder**: Ensure the `pkg` folder generated from `wasm-pack build` is included in your project directory.

2.  **Update Dependencies**: Add the local package to your `package.json`:
    ```json
    "dependencies": {
      "wasm-game-of-life": "file:./pkg"
    }
    ```

3.  **Create React Component**: Create a React component that imports and uses the WASM module to render the simulation.

4.  **Add Vite Plugins**: Configure `vite.config.ts` with `vite-plugin-wasm` and `vite-plugin-top-level-await` to handle WASM files correctly.

5.  **Install Additional Packages**: Install the necessary dependencies, including `rollup`, `vite-plugin-wasm`, `vite-plugin-top-level-await`, and the dev dependency `esbuild`:
    ```bash
    npm install -D rollup vite-plugin-wasm vite-plugin-top-level-await
    npm install -D esbuild
    ```
