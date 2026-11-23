# cargo owner

管理套件（crate）在註冊表上的所有者。

## Usage
- `cargo owner -a 使用者名稱|github:機構名稱:團隊名稱 套件名稱` : 邀請指定的使用者或團隊作為所有者
- `cargo owner -r 使用者名稱|github:機構名稱:團隊名稱 套件名稱` : 將指定的使用者或團隊從所有者中刪除
- `cargo owner -l 套件名稱` : 列出一個套件（crate）的所有者
- `cargo owner --registry 名稱` : 使用指定的註冊表（註冊表名稱可以在配置中定義，預設為 <https://crates.io>）

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-owner.html
