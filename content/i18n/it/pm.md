# pm

Strumento Android pacchetto Manager.

## Usage
- `pm list packages` : elenca i pacchetti installati
- `pm install /percorso/all/app.apk` : installa un pacchetto app da un percorso specificato
- `pm uninstall pacchetto` : disinstalla un pacchetto dal dispositivo
- `pm clear pacchetto` : cancella tutti i dati dell'app per un pacchetto
- `pm enable pacchetto_or_classe` : abilita un pacchetto o componente
- `pm disable-user pacchetto_or_classe` : disabilita un pacchetto o componente
- `pm grant pacchetto android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : concede un permesso per un'app
- `pm revoke pacchetto android.permission.CAMERA|android.permission.ACCESS_FINE_LOCATION|android.permission.READ_CONTACTS|...` : revoca un permesso per un'app

## Links
- More information: https://developer.android.com/tools/adb#pm
