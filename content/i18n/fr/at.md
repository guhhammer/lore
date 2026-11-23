# at

Planifie l'exécution d'une commande une fois à un moment donné. Le service atd (ou atrun) doit être actif pour l'exécution des commandes planifiées.

## Usage
- `systemctl start atd` : démarre `atd` comme un service (en tâche de fond)
- `at now + 5 minutes` : planifie l'exécution de la commande donnée dans l'entrée standard dans 5 minutes (Appuyer sur `<Ctrl d>` une fois la commande inscrite)
- `at hh:mm` : crée des commandes de manière interactive et les exécute à un moment précis
- `echo "commande" | at 1000` : planifie l'exécution d'une commande depuis l'entrée standard (impression echo redirigée dans un tube) aujourd'hui à 10h00
- `at -f chemin/vers/fichier 9:30 PM Tue` : planifie l'exécution des commandes inclues dans un fichier pour mardi prochain 21h30

## Links
- More information: https://manned.org/at
