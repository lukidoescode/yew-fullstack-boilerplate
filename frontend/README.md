# Web Client Frontend

This template serves as a starting point for using Yew in an easy and pragmatic manner. It hopes to serve you as a showcase of what can be done and to provide you with a useful basis for developing your own projects.

## About

This template is loosely based upon the [yew-wasm-pack-template](https://github.com/yewstack/yew-wasm-pack-template).

## 🚴 Usage

Generally you will want to work with the full stack instead, but here are a few instructions to work the web client on its own.

### 🛠️ Build with `yarn run build`

```
yarn run build
```

### 🔬 Serve locally with `yarn run start:dev`

```
yarn run start:dev
```

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
