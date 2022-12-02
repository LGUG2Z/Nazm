set shell := ["cmd.exe", "/C"]
export RUST_BACKTRACE := "full"

clean:
    cargo clean

fmt:
    cargo +nightly fmt
    cargo +stable clippy
    prettier --write README.md

install:
    cargo +stable install --path . --locked

run command:
    cargo +stable run -- {{ command }}