# git cherry-pick

Applica al ramo corrente le modifiche introdotte da commit esistenti. Per applicare le modifiche ad un altro ramo, usa prima `git checkout` per passare al ramo desiderato.

## Usage
- `git cherry-pick commit` : applica un commit al ramo corrente
- `git cherry-pick commit_iniziale~..commit_finale` : applica una sequenza di commit al ramo corrente (vedi anche `git rebase --onto`)
- `git cherry-pick commit_1 commit_2 ...` : applica un insieme di commit non sequenziali al ramo corrente
- `git cherry-pick -n commit` : aggiungi le modifiche introdotte da un commit alla directory di lavoro, ma senza creare un nuovo commit

## Links
- More information: https://git-scm.com/docs/git-cherry-pick
