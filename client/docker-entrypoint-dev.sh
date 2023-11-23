echo "CLient Start!\n"
# diesel 사용시 --database-url 생략하기 이ㅜ한 옵션
rustup target add wasm32-unknown-unknown
cargo install trunk
# cargo watch -x run

trunk serve --address 0.0.0.0 --port ${TRUNK_SERVE_PORT:-8080}