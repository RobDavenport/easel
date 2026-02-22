# demo-wasm

WASM wrapper and static showcase for `easel`.

## Build WASM package

```bash
cd demo-wasm
wasm-pack build --target web --release
```

## Serve showcase

```bash
cd demo-wasm/www
python -m http.server 8080
```
