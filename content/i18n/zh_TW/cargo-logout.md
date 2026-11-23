# cargo logout

將來自註冊表的 API 權杖從本地移除。 該權杖用於對套件註冊表進行身分驗證。您可以使用 `cargo login` 將其添加回來。

## Usage
- `cargo logout` : 從本地憑證儲存中（位於 `$CARGO_HOME/credentials.toml`）移除 API 權杖
- `cargo logout --registry 名稱` : 使用指定的註冊表（註冊表名稱可以在設定中定義，預設為 <https://crates.io>）

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-logout.html
