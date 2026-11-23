# trunk.rs

Bundle and serve Rust WASM web applications.

## Usage
- `trunk serve --release` : build the application in release mode and serve it locally
- `trunk serve -p port` : build the application and serve it on a specific port
- `trunk build --release -d path/to/distribution` : build for production at a specific output directory
- `trunk build --release --public-url /path/to/app_subdirectory` : build with a specific public URL path for hosting in a subdirectory
- `trunk clean` : clean the output directory

## Categories
development, network, files

## Links
- More information: https://trunkrs.dev/commands/
- tldr page: https://tldr.inbrowser.app/pages/common/trunk.rs
