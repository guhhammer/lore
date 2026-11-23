# arch-chroot

Erweiterter `chroot`-Befehl zur Unterstützung des Arch-Linux-Installationsprozesses.

## Usage
- `arch-chroot pfad/zu/neuem/root` : starte eine interaktive Shell (Standardmäßig Bash) in einem neuen Root-Verzeichnis
- `arch-chroot -u anderer_benutzer pfad/zu/neuem/root` : spezifiziere den Benutzer (nicht der jetzige Benutzer) der die Shell ausführt
- `arch-chroot pfad/zu/neuem/root befehl befehlsparameter` : führe einen benutzerdefinierten Befehl (anstelle des Standardbefehls Bash) im neuen Root-Verzeichnis aus
- `arch-chroot pfad/zu/neuem/root zsh` : gib die Shell an, die nicht die Standard-Shell Bash ist (in diesem Fall sollte das Paket `zsh` auf dem Zielsystem installiert worden sein)

## Links
- More information: https://manned.org/arch-chroot
