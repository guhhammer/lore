# mvn

Apache Maven：Java ベースのプロジェクトをビルドおよび管理します。

## Usage
- `mvn compile` : プロジェクトをコンパイルする
- `mvn package` : コンパイルしたコードを `jar` のような配布可能な形式でパッケージ化
- `mvn package -D skipTests` : ユニットテストをスキップしてコンパイルし、パッケージ化
- `mvn install` : ビルドしたパッケージをローカルの maven リポジトリにインストール (コンパイルとパッケージのコマンドも呼び出される)
- `mvn clean` : ターゲットディレクトリから、ビルドアーティファクトを削除
- `mvn clean package` : clean にしてから package フェーズを起動
- `mvn clean -P プロファイル package` : 指定されたビルドプロファイルを使用して、コードをクリーンアップしてパッケージ化
- `mvn exec:java -D exec.mainClass="com.example.Main" -D exec.args="引数1 引数2 ..."` : main メソッドを持つクラスを実行

## Links
- More information: https://manned.org/mvn
