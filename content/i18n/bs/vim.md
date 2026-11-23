# vim

Vim (Vi IMproved), je uređivač teksta za komandnu liniju koji nudi više režima rada za različite vrste manipulacije tekstom. Pritiskom na `<i>` u normalnom režimu prelazi se u režim umetanja. Pritiskom na `<Esc>` vraćate se u normalni režim, koji omogućava korištenje Vim komande.

## Usage
- `vim put/do/datoteke` : otvorite datoteku
- `vim +broj_linije put/do/datoteke` : otvorite datoteku na oređenoj liniji
- `<:>help<Enter>` : pogledajte pomoćni list za Vim
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : sačuvaj i zatvori trenutni buffer
- `<Esc><u>` : prebaci se u normalni režim i opozovi posljednu operaciju
- `</>uzorak_pretrage<Enter>` : tražite uzorak u datoteci (pritiskom na `<n>`/`<N>` prelazite na sljedeće/predprošlo podudaranje)
- `<:>%s/regex/zamjena/g<Enter>` : izvedite `regex` (regularni izraz) zamjenu u cijeloj datoteci
- `<:>set nu<Enter>` : pokaži brojeve linija

## Links
- More information: https://www.vim.org/
