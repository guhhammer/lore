# cargo metadata

以 JSON 格式輸出目前套件的工作空間成員和已解析的相依項。 注意：輸出格式可能在未來的 Cargo 版本中發生變化。

## Usage
- `cargo metadata` : 列印目前套件的工作空間成員和已解析的相依項
- `cargo metadata --no-deps` : 僅列印工作空間成員，不取得相依項
- `cargo metadata --format-version 版本號` : 根據指定版本列印特定格式的元資料
- `cargo metadata --filter-platform 目標三元組` : 列印帶有 `resolve` 欄位的元資料，僅包含給定目標三元組的相依項（注意：`packages` 陣列仍將包含所有目標的相依項）

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
