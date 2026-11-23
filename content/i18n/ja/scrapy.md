# scrapy

ウェブクローリングのフレームワークです。

## Usage
- `scrapy startproject プロジェクト名` : プロジェクトを作成する
- `scrapy genspider スパイダー名 ウェブサイトのドメイン名` : スパイダーを作成する (プロジェクトのディレクトリ内での実行)
- `scrapy edit スパイダー名` : スパイダーを編集する (プロジェクトのディレクトリ内での実行)
- `scrapy crawl スパイダー名` : スパイダーを実行する (プロジェクトのディレクトリ内での実行)
- `scrapy fetch url` : scrapyが見るようにWebページを取得しソースを`stdout`(標準出力)に表示する
- `scrapy view url` : scrapyが見ているようにデフォルトブラウザ内でウェブページを開く(より応答に忠実であるようにするためにJavaScriptを無効化している)
- `scrapy shell url` : URL用のScrapyシェルを開き、Python(もしくは可能であればIPython)シェル内でページソースとの対話式でのやり取りを可能にする

## Links
- More information: https://docs.scrapy.org/en/latest/topics/commands.html#using-the-scrapy-tool
