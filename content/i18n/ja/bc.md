# bc

任意の精度で計算を行える言語です。

## Usage
- `bc` : 対話モードのセッションを開始する
- `bc --mathlib` : 標準数学ライブラリを有効化して対話モードのセッションを開始する
- `echo '5 / 3' | bc` : 式を計算する
- `bc スクリプト/への/パス.bc` : スクリプトを実行する
- `echo 'scale = 10; 5 / 3' | bc` : 小数点以下の桁数を指定して式を計算する
- `echo 's|c|a|l|e(1)' | bc --mathlib` : `mathlib`を使用してサイン/コサイン/アークタンジェント/自然対数/指数関数を計算する

## Links
- More information: https://manned.org/bc
