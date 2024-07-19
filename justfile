import? 'local.justfile'
all:
  just --list
t:
  cargo test
b:
  cargo build

source-add-npm-workspaces:
  npmpink source add ./assets_/fixtures_npm_workspaces
