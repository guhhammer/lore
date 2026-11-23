# git rebase

Wende Commits von einem Branch auf einen anderen Branch an. Die Änderungen eines Branches werden auf einen bestehenden Branch "übertragen" und am Ende der Historie als neue Commits eingefügt.

## Usage
- `git rebase neuer_basisbranch` : verwende einen anderen, angegebenen Branch als Basis für den aktuellen Branch
- `git rebase -i ziel_basisbranch_oder_commithash` : starte einen interaktiven Rebase, bei dem Commits umsortiert, weggelassen, kombiniert oder verändert werden können
- `git rebase --continue` : setze einen Rebase fort, der durch einen Mergefehler unterbrochen wurde, nachdem die Konflikte aufgelöst wurden
- `git rebase --skip` : setze einen Rebase fort, der durch einen Mergefehler unterbrochen wurde, durch Auslassen des in Konflikt stehenden Commits
- `git rebase --abort` : brich einen laufenden Rebase ab (z.B. wenn er durch Mergekonflikte unterbrochen wurde)
- `git rebase --onto neue_basis alte_basis` : verschiebe einen Teil des aktuellen Branches auf eine neue Basis und gib die alte Basis an, ab der die Änderungen verwendet werden sollen
- `git rebase -i HEAD~5` : bearbeite die 5 letzten Commits der aktuellen Basis um diese neu zu ordnen, auszulassen, kombinieren oder zu bearbeiten
- `git rebase -X theirs branch_name` : löse Konflikte automatisch auf, indem der aktuelle Branch bevorzugt wird (das Schlüsselwort `theirs` hat in diesem Fall eine umgekehrte Bedeutung)

## Links
- More information: https://git-scm.com/docs/git-rebase
