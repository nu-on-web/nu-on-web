build:
	wasm-pack build --dev --no-opt --no-pack --target web --out-dir playground/src/wasm

prod:
	wasm-pack build --target web --no-pack --release --out-dir playground/src/wasm
