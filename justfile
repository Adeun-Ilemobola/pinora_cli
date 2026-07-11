build:
    cargo build

install:
    cargo build
    cargo install --path .

release:
    cargo build --release
    cargo install --path . --force