# topydo

Une application de liste de choses à faire qui utilise le format todo.txt.

## Usage
- `topydo add "message_todo +nom_projet @nom_contexte"` : ajoute une tâche à un projet spécifique avec un contexte donné
- `topydo add "(A) message_todo due:1d"` : ajoute une tâche à faire avec une date d'échéance de demain et une priorité de `A`
- `topydo add "message_todo due:fri"` : ajoute une tâche à faire dont la date d'échéance est le vendredi
- `topydo add "water flowers due:mon rec:1w"` : ajoute une tâche répétitive non stricte (jour + récurrence)
- `topydo add "message_todo due:2020-01-01 rec:+1m"` : ajoute une tâche répétitive stricte (prochaine échéance = date + récurrence)
- `topydo revert` : revient sur la dernière commande `topydo` exécutée

## Links
- More information: https://github.com/topydo/topydo
