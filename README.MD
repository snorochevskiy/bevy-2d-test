# Bevy 2D top down shooter game example

Make sure you have Rust SDK installed.

## Launch game from sources

```
cargo run --release
```

## Cross compile for windows

For deb distributions:

```shell
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```