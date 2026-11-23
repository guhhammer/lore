# rustup-init.sh

Script to install `rustup` and the Rust toolchain.

## Usage
- `curl https://sh.rustup.rs -sSf | sh -s` : download and run `rustup-init` to install `rustup` and the default Rust toolchain
- `curl https://sh.rustup.rs -sSf | sh -s -- arguments` : download and run `rustup-init` and pass arguments to it
- `rustup-init.sh --target target --component component` : run `rustup-init` and specify additional components or targets to install
- `rustup-init.sh --default-toolchain toolchain` : run `rustup-init` and specify the default toolchain to install
- `rustup-init.sh --default-toolchain none` : run `rustup-init` and do not install any toolchain
- `rustup-init.sh --profile minimal|default|complete` : run `rustup-init` and specify an installation profile
- `rustup-init.sh -y` : run `rustup-init` without asking for confirmation

## Categories
packages, network, development

## Links
- More information: https://forge.rust-lang.org/infra/other-installation-methods.html#rustup
- tldr page: https://tldr.inbrowser.app/pages/common/rustup-init.sh
