# git archive

ツリーからファイルのアーカイブを作成する。

## Usage
- `git archive -v HEAD` : 現在の `HEAD` の内容から `.tar` アーカイブを作成して `stdout` に出力する
- `git archive -v --format zip HEAD` : zip形式を使用して進捗を詳細に表示する
- `git archive -v -o ディレクトリ/サブディレクトリ/ファイル.zip HEAD` : zipアーカイブを指定したファイルへ出力する
- `git archive -o ディレクトリ/サブディレクトリ/ファイル.tar ブランチ名` : 指定したブランチの最新コミットの内容から `.tar` アーカイブを作成する
- `git archive -o ディレクトリ/サブディレクトリ/ファイル.tar HEAD:ディレクトリ/サブディレクトリ` : 指定したディレクトリの内容を使用する
- `git archive -o ディレクトリ/サブディレクトリ/ファイル.tar --prefix 付加する/パス/ HEAD` : 各ファイルのパスの先頭にパスを付加してアーカイブ内の指定したディレクトリに格納する

## Links
- More information: https://git-scm.com/docs/git-archive
