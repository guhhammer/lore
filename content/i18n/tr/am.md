# am

Android aktivite yöneticisi.

## Usage
- `am start -n com.android.settings/.Settings` : belirtilmiş bir aktiviteyi başlat
- `am start -a android.intent.action.VIEW -d tel:123` : bir aktivite başlatıp veriyi ona aktar
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : belirtilmiş bir eylem ve kategoriyi karşılayan bir aktivite başlat
- `am to-uri -a android.intent.action.VIEW -d tel:123` : bir kastı URI'a çevir

## Links
- More information: https://developer.android.com/tools/adb#am
