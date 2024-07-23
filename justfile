import? 'local.justfile'
all:
  just --list
t:
  cargo test
b:
  cargo build

source-add-npm-workspaces:
  npk source add ./assets_/fixtures_npm_workspaces
view-lock:
  bat npmpink.lock
