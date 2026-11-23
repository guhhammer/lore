# make

Makefile に記述されたターゲットのタスクランナー。 主に、ソースコードから実行可能ファイルのコンパイルを制御する為に使用されます。

## Usage
- `make` : makefile で指定された最初のターゲットを呼び出す (通常 "all" という名前)
- `make ターゲット` : 特定のターゲットを呼び出す
- `make -j 4 ターゲット` : 特定のターゲットを呼び出し、一度に 4 つのジョブを並列実行
- `make -f path/to/file` : 指定した Makefile を使用
- `make -C path/to/directory` : 別ディレクトリから make を実行
- `make -B ターゲット` : ソースファイルが変更されていなくても、強制的にターゲットを作る
- `make ターゲット 変数名=値` : makefile で定義された変数を上書きする
- `make -e ターゲット` : 環境変数によって与えられた変数で、 Makefile に定義されている変数を上書きするようにする

## Links
- More information: https://www.gnu.org/software/make/manual/make.html
