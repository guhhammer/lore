# cargo fix

Automatically fix lint warnings reported by `rustc`.

## Usage
- `cargo fix --broken-code` : fix code even if it already has compiler errors
- `cargo fix --allow-dirty` : fix code even if the working directory has changes
- `cargo fix --edition` : migrate a package to the next Rust edition
- `cargo fix --lib` : fix the package's library
- `cargo fix --test name` : fix the specified integration test
- `cargo fix --workspace` : fix all members in the workspace

## Categories
development

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-fix.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-fix
- Wikipedia: https://en.wikipedia.org/wiki/Rust_(programming_language)
