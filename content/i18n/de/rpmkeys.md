# rpmkeys

Werkzeug um RPM Schüssel für RPM Repositorien zu importieren oder löschen. Wenn ein RPM Repository hinzugefügt wird, dann muss man auch den entsprechend RPM Schüssel importieren.

## Usage
- `sudo rpmkeys --list` : list alle importierte RPM Schüssel auf. Git auch die Schüssel ID aus welche beim späteren Löschen gebraucht wird
- `sudo rpmkeys --delete 5a278d9c-5bbc73cb` : entfern/Lösch eine zuvor importierten RPM Schüssel, angeben durch eine 16 stellige Nummer/Buchstaben Folge
- `sudo rpmkeys --import pfad/zu/rpm_schüssel` : importier einen RPM Schüssel eines Repositorien

## Links
- More information: https://rpm-software-management.github.io/rpm/man/rpmkeys.8
