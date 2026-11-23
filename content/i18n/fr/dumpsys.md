# dumpsys

Fournit des informations sur les services du système Android. Cette commande peut être utilisé uniquement depuis `adb shell`.

## Usage
- `dumpsys` : récupère un diagnostic pour chaque service système
- `dumpsys service` : récupère un diagnostic pour un service système spécifique
- `dumpsys -l` : liste tous les services dont `dumpsys` peut donner les informations
- `dumpsys service -h` : liste les arguments spécifiques d'un service
- `dumpsys --skip service` : exclut un service spécifique d'un diagnostic
- `dumpsys -t seconds` : spécifie un temps limite en secondes (10s par défaut)

## Links
- More information: https://developer.android.com/tools/dumpsys
