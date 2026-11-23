# gcc

C と C++ のソースファイルを前処理してコンパイルし、アセンブルしてリンクします。 GCC (GNU Compiler Collection) の一部です。

## Usage
- `gcc path/to/source1.c path/to/source2.c ... -o path/to/output_executable` : 複数のソースファイルを実行ファイルにコンパイルする
- `gcc path/to/source.c -Wall -o output_executable` : 全てのエラーと警告の出力を有効にする
- `gcc path/to/source.c -Wall -g -Og -o path/to/output_executable` : 一般的な警告、デバッグシンボルを出力に表示し、デバッグに影響を与えずに最適化する
- `gcc path/to/source.c -o path/to/output_executable -Ipath/to/header -Lpath/to/library -llibrary_name` : 別のパスからライブラリをインクルード
- `gcc -S path/to/source.c` : ソースコードをアセンブラ命令にコンパイルする
- `gcc -c path/to/source.c` : ソースコードをリンクせずにオブジェクトファイルにコンパイル
- `gcc path/to/source.c -O1|2|3|fast -o path/to/output_executable` : コンパイルしたプログラムをパフォーマンスのために最適化する
- `gcc --version` : バージョンを表示

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
