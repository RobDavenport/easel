# demo-wasm

WASM wrapper and static showcase for `easel`.

## Build WASM package

```bash
wasm-pack build demo-wasm --target web --release --out-dir www/pkg
```

## Serve showcase

```bash
cd demo-wasm/www
python -m http.server 8080
```

Then open `http://localhost:8080` and verify the WASM status line reports `ready`.
