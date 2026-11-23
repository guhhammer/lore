# git commit-graph

Gitのcommit-graphファイルを書き込み、検証する。

## Usage
- `git commit-graph write` : リポジトリのローカル `.git` ディレクトリ内にあるパック済みコミット用のcommit-graphファイルを書き込む
- `git show-ref -s | git commit-graph write --stdin-commits` : 到達可能なすべてのコミットを含むcommit-graphファイルを書き込む
- `git rev-parse HEAD | git commit-graph write --stdin-commits --append` : 現在のcommit-graphファイル内のすべてのコミットと、`HEAD` から到達可能なコミットを含むcommit-graphファイルを書き込む

## Links
- More information: https://git-scm.com/docs/git-commit-graph
