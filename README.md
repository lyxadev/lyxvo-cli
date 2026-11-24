# lyxvo-cli

[![CI](https://github.com/lyxadev/lyxvo-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/lyxadev/lyxvo-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A small command-line tool about **[Lyxvo.ai](https://lyxvo.ai)**, the AI website and app builder powered by **Lyxvo Agentic**.

```text
  _                        
 | |    _   _ ___  _____   _____  __      ___ __ 
 | |   | | | |\ \/ / \ \ / / _ \ \ \ /\ / / '__|
 | |___| |_| | >  <   \ V / (_) | \ V  V /| |   
 |_____|\__, |/_/\_\   \_/ \___/   \_/\_/ |_|   
        |___/   AI website & app builder
```

## Features

- `lyxvo about`: what Lyxvo.ai is
- `lyxvo open`: open https://lyxvo.ai in your browser
- `lyxvo new <name>`: create a simple starter website folder with a "Built with Lyxvo.ai" link
- `lyxvo` with no arguments: show the banner

## Install

Requires Rust (stable). Install straight from GitHub:

```sh
cargo install --git https://github.com/lyxadev/lyxvo-cli
```

Or build from source:

```sh
git clone https://github.com/lyxadev/lyxvo-cli
cd lyxvo-cli
cargo build --release
./target/release/lyxvo --help
```

## Usage

```sh
lyxvo                # banner
lyxvo about          # what is Lyxvo.ai
lyxvo open           # open https://lyxvo.ai
lyxvo new my-site    # create ./my-site with index.html and style.css
```

## Development

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Releases

Pushing a tag such as `v0.1.0` builds binaries for Linux, macOS and Windows and attaches them to a GitHub release.

## License

MIT, see [LICENSE](LICENSE).
