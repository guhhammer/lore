# am

Manajer aktivitas untuk Android.

## Usage
- `am start -n com.android.settings/.Settings` : mulaikan aktivitas tertentu
- `am start -a android.intent.action.VIEW -d tel:123` : mulaikan aktivitas dengan data yang ditentukan
- `am start -a android.intent.action.MAIN -c android.intent.category.HOME` : mulaikan aktivitas dengan aksi dan kategori tertentu
- `am to-uri -a android.intent.action.VIEW -d tel:123` : ubah sebuah Intent menjadi tautan URI

## Links
- More information: https://developer.android.com/tools/adb#am
