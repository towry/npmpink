import? 'local.justfile'
all:
  just --list
t:
  cargo test
b:
  cargo build
build-release:
  cargo build --release
install: build-release
  cp ./target/release/npk /usr/local/bin
install-debug:
  ln -sf $(pwd)/target/debug/npk /usr/local/bin/npk

source-add-npm-workspaces:
  npk source add ./assets_/fixtures_npm_workspaces
view-lock:
  bat npmpink.lock
