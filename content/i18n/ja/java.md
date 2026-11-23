# java

Java アプリケーションランチャ。

## Usage
- `java クラス名` : main メソッドを含む Java の `.class` ファイルを、クラス名だけで実行する
- `java -classpath path/to/classes1:path/to/classes2:. クラス名` : java プログラムを実行し、追加のサードパーティまたはユーザー定義クラスを使用する
- `java -jar filename.jar` : `.jar` プログラムを実行する
- `java -agentlib:jdwp=transport=dt_socket,server=y,suspend=y,address=*:5005 -jar filename.jar` : `.jar` プログラムを、デバッグ待ちポート 5005 で実行する
- `java -help` : ヘルプを表示する
- `java -version` : JDK、JRE、HotSpot のバージョンを表示する

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/java.html
