# nu-on-web

A WebAssembly port of [NuShell](https://www.nushell.sh), enabling you to run NuShell pipelines directly in your browser.

## Demo

Explore the live demo here: https://nu-on-web.github.io/nu-on-web/

## Features

- Evaluate NuShell code in the browser using a WASM-powered engine.
- Core commands: `ls`, `cat`, `rm`, and more via [nu-cmd-extra](https://crates.io/crates/nu-cmd-extra).
- Command auto-completion and descriptions for IDE-like experience.
- Virtual filesystem in the browser powered by [ZenFS](https://github.com/zenfs/core).
- Frontend built with Svelte, Vite, and Monaco Editor.
- Easy setup with `make`, `bun`, or Nix flake.

## Getting Started

### Prerequisites

- Rust (with `wasm32-unknown-unknown` target)
- `wasm-pack`
- Bun (or Node.js with npm)
- Alternatively, use [Nix flakes](https://nixos.org/manual/nix/latest/command-ref/nix3-flake.html):
  ```bash
  nix develop
  ```

### Build

```bash
# Development build (unoptimized)
make build

# Production build (optimized)
make prod
```

### Run the Playground

```bash
cd playground
bun install
bun run dev
```

Open http://localhost:5173 in your browser.

## Testing

Currently, there are no automated tests for this project. Contributions adding tests are welcome!

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT or Apache-2.0 License. See [LICENSE_MIT](LICENSE_MIT) and [LICENSE_APACHE](LICENSE_APACHE) for details.
