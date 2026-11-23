# adb install

Android Debug Bridge Install: Pousse des paquets vers une instance d'émulateur Android ou un appareil Android.

## Usage
- `adb install chemin/vers/fichier.apk` : pousse une application Android vers l'émulateur/l'appareil
- `adb -s numéro_série install chemin/vers/fichier.apk` : pousse une application Android vers l'émulateur/l'appareil spécifique via son numéro de série (écrase la variable `$ANDROID_SERIAL`)
- `adb install -r chemin/vers/fichier.apk` : réinstalle une application existante, tout en gardant ses données
- `adb install -d chemin/vers/fichier.apk` : pousse une application Android en autorisant la rétrogradation de version (uniquement pour les paquets debuggable)
- `adb install -g chemin/vers/fichier.apk` : accorde toutes les permissions listées dans le manifeste de l'application
- `adb install --fastdeploy chemin/vers/fichier.apk` : met à jour rapidement un paquet en mettant à jour uniquement les parties de l'APK qui ont changé

## Links
- More information: https://developer.android.com/tools/adb
