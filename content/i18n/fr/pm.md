# pm

Gestionnaire de Paquets d'Android.

## Usage
- `pm list packages` : liste les paquets installés
- `pm install /chemin/vers/fichier.apk` : installe une application depuis un chemin donné
- `pm uninstall paquet` : déinstalle un paquet de l'appareil
- `pm clear paquet` : supprime les données d'une application
- `pm enable paquet_ou_classe` : active une application ou un composant
- `pm disable-user paquet_ou_classe` : désactive une application ou un composant
- `pm grant paquet android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : donne une permission à une application
- `pm revoke paquet android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : révoque une permission d'une application

## Links
- More information: https://developer.android.com/tools/adb#pm
