# adb

Android Debug Bridge: Communiquer avec une instance d'émulateur Android ou un appareil Android. Certaines commandes comme `adb shell` ont leur propre documentation.

## Usage
- `adb start-server` : vérifie si le processus du serveur adb est en fonctionnement et le démarre
- `adb kill-server` : arrête le processus du serveur adb
- `adb shell` : démarre un shell distant sur l'émulateur/l'appareil ciblé
- `adb install -r chemin/vers/fichier.apk` : pousse une application Android vers l'émulateur/l'appareil
- `adb pull chemin/vers/le/fichier_ou_dossier_de_l'appareil chemin/vers/le/dossier_de_destination_local` : copie un fichier/dossier depuis l'appareil ciblé
- `adb push chemin/vers/le/fichier_ou_dossier_local chemin/vers/le/dossier_de_destination_de_l'appareil` : copie un fichier/dossier vers l'appareil ciblé
- `adb devices` : récupère une liste des appareils connectés

## Links
- More information: https://developer.android.com/tools/adb
