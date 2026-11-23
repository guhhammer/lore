# rustc

Rust 編譯器。 Rust 專案通常使用 `cargo` 而不是直接呼叫 `rustc`。

## Usage
- `rustc 路徑/至/主檔案.rs` : 編譯二進位套件（crate）
- `rustc -C lto -C opt-level=0|1|2|3|s|z 路徑/至/主檔案.rs` : 編譯時啟用最佳化（`s` 表示針對二進位大小最佳化；`z` 含義相同但使用更多最佳化）
- `rustc -g 路徑/至/主檔案.rs` : 編譯時包含除錯資訊
- `rustc --explain 錯誤碼` : 解釋錯誤資訊
- `rustc -C target-cpu=native 路徑/至/主檔案.rs` : 編譯時針對目前 CPU 架構啟用特定最佳化
- `rustc --print target-list` : 顯示目標列表（注意：你必須先使用 `rustup` 新增目標才能為其編譯）
- `rustc --target 目標三元組 路徑/至/主檔案.rs` : 針對特定目標編譯
- `rustc -h` : 顯示說明

## Links
- More information: https://doc.rust-lang.org/stable/rustc/
