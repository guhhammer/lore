# git subtree

サブツリーをまとめてマージ、またはリポジトリからサブツリーを分割する。

## Usage
- `git subtree add -P ディレクトリ/サブディレクトリ --squash リポジトリURL ブランチ名` : gitリポジトリをサブツリーとして追加し、コミットをまとめて1つにする
- `git subtree pull -P ディレクトリ/サブディレクトリ リポジトリURL ブランチ名` : サブツリーのリポジトリを最新コミットへ更新する
- `git subtree merge -P ディレクトリ/サブディレクトリ --squash リポジトリURL ブランチ名` : 最新のサブツリーコミットまでの変更をサブツリーへマージする
- `git subtree push -P ディレクトリ/サブディレクトリ リポジトリURL ブランチ名` : コミットをサブツリーのリポジトリへプッシュする
- `git subtree split -P ディレクトリ/サブディレクトリ リポジトリURL -b ブランチ名` : サブツリーの履歴から新しいプロジェクトの履歴を抽出する

## Links
- More information: https://manned.org/git-subtree
