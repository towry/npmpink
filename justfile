import? 'local.justfile'
all:
  just --list
t:
  cargo test
b:
  cargo build
