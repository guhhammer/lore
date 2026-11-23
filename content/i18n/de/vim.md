# vim

Vim (Vi IMproved), ein Befehlszeilen-Texteditor, bietet mehrere Modi für verschiedene Arten der Textmanipulation an. Das Drücken von `<i>` schaltet den Editier-Modus ein. `<Esc>` wechselt in den Befehls-Modus, der die Verwendung von Vim-Befehlen ermöglicht.

## Usage
- `vim pfad/zu/datei` : öffne eine Datei
- `vim +zeilennummer pfad/zu/datei` : öffne eine Datei an einer bestimmten Zeilennummer
- `<:>help<Enter>` : zeige Vim's Benutzeranleitung
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : speichere und schließe die aktuelle Datei
- `<Esc><u>` : mache die letzte Aktion rückgängig
- `</>suchmuster<Enter>` : suche nach einem Muster in der Datei (mit `<n>`/`<N>` zum nächsten/vorherigen Treffer gehen)
- `<:>%s/regulärer_ausdruck/neuer_text/g<Enter>` : ersetze einen regulären Ausdruck alle Treffer in einer Datei
- `<:>set nu<Enter>` : zeige Zeilennummern an

## Links
- More information: https://www.vim.org/
