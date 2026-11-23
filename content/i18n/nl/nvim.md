# nvim

Neovim, een programmeurstekstbewerker gebaseerd op Vim, welke verschillende modi aanbiedt voor verschillende soorten tekstmanipulatie. Op `<i>` drukken in de normale modus, gaat naar de invoermodus. `<Esc>` of `<Ctrl c>` gaat terug naar de normale modus, die geen reguliere tekstinvoer accepteert.

## Usage
- `nvim pad/naar/bestand` : open een bestand
- `<Esc><i>` : ga naar de modus om tekst aan te passen (insert mode)
- `<Esc><y><y>|<d><d>` : kopieer ("yank") of knip ("delete") de huidige regel (plak het met `<p>`)
- `<Esc><u>` : ga naar de normale modus en maak de laatste operatie ongedaan
- `<Esc></>zoek_patroon<Enter>` : zoek naar een patroon in het bestand (druk op `<n>`/`<N>` om naar de volgende/vorige overeenkomst te gaan)
- `<Esc><:>%s/regex/vervanging/g<Enter>` : voer een `regex` vervanging uit in het volledige bestand
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : ga naar de normale modus, sla (write) het bestand op en sluit af
- `<Esc><:>q!<Enter>` : sluit af zonder op te slaan

## Links
- More information: https://neovim.io/
