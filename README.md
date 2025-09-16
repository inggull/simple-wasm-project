# A Simple WebAssembly Project

러스트로 빌드한 웹어셈블리 라이브러리를 자바스크립트에 적용해보는 간단한 프로젝트입니다.

## 실행 순서

```bash
rustup target add wasm32-unknown-unknown
cargo build --package server
cargo build --package add --target wasm32-unknown-unknown
cargo run --bin server
```

Localhost url: <http://127.0.0.1:8080/index.html>
