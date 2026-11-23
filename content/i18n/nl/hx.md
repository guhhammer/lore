# hx

Helix, een postmoderne tekstbewerker, welke verschillende modi beschikbaar stelt voor verschillende manieren van tekstmanipulatie. Drukken op `<i>` begint invoegmodus. `<Esc>` begint normale modus, wat toegang geeft tot de Helix commando's.

## Usage
- `hx pad/naar/bestand` : open een bestand
- `hx --vsplit pad/naar/bestand1 pad/naar/bestand2 ...` : open bestanden en toon ze naast elkaar
- `hx --tutor` : toon de tutorial om Helix te leren (of open het binnen Helix door op `<Esc>` te drukken en `<:>tutor<Enter>` te typen)
- `<:>theme thema_naam` : pas het Helix thema aan
- `<:>wq<Enter>` : sla op en sluit af
- `<:>q!<Enter>` : sluit geforceerd af zonder op te slaan
- `<u>|<U>` : maak de laatste verandering ongedaan/opnieuw
- `</>zoek_patroon<Enter>` : zoek een patroon in het bestand (druk op `<n>`/`<N>` om naar de volgende/vorige overeenkomst te gaan)

## Links
- More information: https://manned.org/man/debian-forky/hx
