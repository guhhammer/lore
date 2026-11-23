# am

Android Activity Manager（アクティビティ マネージャー）。

## Usage
- `am start -n com.android.settings/.Settings` : 指定したコンポーネントとパッケージ名でアクティビティを開始します
- `am start -a android.intent.action.VIEW -d tel:123` : インテントアクションを開始させ、データを渡します
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : 指定したカテゴリーとマッチしたアクティビティを実行します
- `am to-uri -a android.intent.action.VIEW -d tel:123` : インテントをURIに変換します

## Links
- More information: https://developer.android.com/tools/adb#am
