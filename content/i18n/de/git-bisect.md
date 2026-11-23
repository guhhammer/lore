# git bisect

Benutzt binäre Suche um den commit ausfindig zu machen, welcher einen Fehler beinhaltet. Git springt im Commit-Graph automatisch vor und zurück, um den fehlerhaften Commit schrittweise eingrenzen zu können.

## Usage
- `git bisect start fehlerhafter_commit sauberer_commit` : starte eine Bisect-Session in einem Commit-Bereich, der durch einen bekannten fehlerhaften Commit und einen sauberen Commit begrenzt wird
- `git bisect good|bad` : prüfe jeden Commit, den `git bisect` auswählt, und kennzeichne ihn mit "gut" oder "schlecht"
- `git bisect reset` : wechsle zum vorherigen Branch zurück, nachdem der fehlerhafte Commit lokalisiert wurde
- `git bisect skip` : überspringe einen Commit während der Bisect-Session (z.B. einen, der die Tests aufgrund eines anderen Problems nicht bestanden hat)

## Links
- More information: https://git-scm.com/docs/git-bisect
