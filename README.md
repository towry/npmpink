![push](https://github.com/towry/npmpink/actions/workflows/ci.yml/badge.svg?event=push) <a href="https://github.com/towry/npmpink/releases/latest"><img src="https://img.shields.io/github/v/release/towry/npmpink?display_name=tag&label=release%3A%20npk" alt="release badge" /></a>


# WIP

## Install

```
cargo install --git https://github.com/towry/npmpink
```

### Install from release

https://github.com/towry/npmpink/releases/

## Features

- linked deps manage (high priority).
- tui (low priority).

## Commands

#### Add directory to source registry.

Source are where the npm packages could be searched.

```
npk source add <dir>
```

#### Add package to project.

Add packages from sources to your project's `npmpink.lock` file, those are the packages
that will be linked into your project by command `npk sync`.

```
cd <your project>
npk package add
```

### Link packages to project.

```
npk sync
```

## TODO

- [x] `npmpink source add`, basic.
- [x] `npmpink source list`, basic.
- [x] `npmpink source remove`, basic.
- [x] `npmpink package add`, basic.
- [x] `npmpink package remove`, basic.
- [x] `npmpink package sync`, basic.
- [x] Better package discovery.
- [x] Error message handle.
- [x] Pretty console output.

- https://excalidraw.com/#json=7oqX_amJ0GwZaldcHYkHp,hKmYmGQI-AHS2k2AMQFReA
- https://docs.rs/tui/0.19.0/tui/
