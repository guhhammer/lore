# vim

Vim (Vi IMproved), edytor tekstu wiersza polecenia, oferuje kilka trybów dla różnych rodzajów manipulacji tekstem. Naciśnięcie `<i>` powoduje przejście do trybu edycji. `<Esc>` wraca do normalnego trybu, który pozwala na używanie komend.

## Usage
- `vim scieżka/do/pliku` : otwórz plik
- `vim +numer_linii scieżka/do/pliku` : otwórz plik pod określonym numerem wiersza
- `<:>help<Enter>` : zobacz instrukcję pomocy Vim
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : wyjdź bez zapisywania
- `<Esc><u>` : cofnij ostatnią operację
- `</>szukaj_wzorca<Enter>` : wyszukaj wzorzec w pliku (naciśnij `<n>`/`<N>` przejść do następnego/poprzedniego dopasowania)
- `<:>%s/wzorzec/zastąpienie/g<Enter>` : wykonaj podstawienie wyrażenia regularnego w całym pliku
- `<:>set nu<Enter>` : wyświetlaj numery linii

## Links
- More information: https://www.vim.org/
