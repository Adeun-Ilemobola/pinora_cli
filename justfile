set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

build:
    cargo build

release:
    cargo build --release

install:
    cargo install --path . --force