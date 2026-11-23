# ghdl

Simulateur à source ouvert pour le langage VHDL.

## Usage
- `ghdl -a fichier.vhdl` : analyse un fichier de source VHDL et génère un fichier objet
- `ghdl -e design` : élabore un design (où `design` est le nom d'une unité de configuration, d'entité, ou d'architecture)
- `ghdl -r design` : exécute un design élaboré
- `ghdl -r design --wave=sortie.ghw` : exécute un design élaboré et sauvegarde la sortie à un fichier de forme d'onde
- `ghdl -s fichier.vhdl` : vérifie le syntaxe d'un fichier de source VHDL
- `ghdl --help` : affiche l'aide

## Links
- More information: https://ghdl.github.io/ghdl/
