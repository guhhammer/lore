# pm

Android Package Manager-hulpprogramma.

## Usage
- `pm list packages` : toon geïnstalleerde pakketten
- `pm install /pad/naar/app.apk` : installeer een app-pakket van een opgegeven pad
- `pm uninstall pakket` : verwijder een pakket van het apparaat
- `pm clear pakket` : verwijder alle app-data voor een pakket
- `pm enable pakket_of_class` : schakel een pakket of component in
- `pm disable-user pakket_of_class` : schakel een pakket of component uit
- `pm grant pakket android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : geef toestemming aan een app
- `pm revoke pakket android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : trek toestemming voor een app in

## Links
- More information: https://developer.android.com/tools/adb#pm
