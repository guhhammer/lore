# cargo update

Update dependencies as recorded in `Cargo.lock`.

## Usage
- `cargo update` : update dependencies in `Cargo.lock` to the latest possible version
- `cargo update -n` : simulate an update, displaying what would be updated without writing the lockfile
- `cargo update --package dependency1 --package dependency2 --package dependency3` : update only the specified dependencies
- `cargo update --package dependency --precise 1.2.3` : set a specific dependency to a specific version

## Categories
packages

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-update.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-update
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
