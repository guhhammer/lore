# tldr

Zeigt kurze Zusammenfassungen (tldr-Seiten) von Kommandozeilen-Befehlen an.

## Usage
- `tldr befehl` : zeige die tldr-Seite für einen Befehl an
- `tldr befehl unterbefehl` : zeige die tldr-Seite für einen Unterbefehl an
- `tldr -L sprachcode befehl` : zeige die tldr-Seite für einen Befehl in einer bestimmten Sprache an (Hinweis: So bist du hierher gekommen!)
- `tldr -p android|common|freebsd|linux|osx|netbsd|openbsd|sunos|windows befehl` : zeige die tldr-Seite für einen Befehl auf einer bestimmten Plattform an
- `tldr -u` : aktualisiere den lokalen Cache
- `tldr -l` : liste alle Seiten für die aktuelle Plattform und `common` auf
- `tldr -l | grep befehl | column` : liste alle Unterbefehl-Seiten für einen Befehl auf
- `tldr -l | shuf -n 1 | xargs tldr` : zeige die tldr-Seite für einen zufälligen Befehl an
