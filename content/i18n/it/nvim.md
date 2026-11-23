# nvim

Neovim, un editor di testo basato su Vim che offre molti diversi modi di manipolare e navigare il testo. Premere `<i>` per entrare in modalità inserimento (insert mode), `<Esc>` per uscire e tornare alla modalità normale (normal mode).

## Usage
- `nvim percorso/del/file` : aprire un file
- `<Esc><i>` : entrare nella modalità per scrivere testo (insert mode)
- `<Esc><y><y>|<d><d>` : copiare ("yank") o cancellare ("delete") la linea corrente (può poi essere copiata con `<p>`)
- `<Esc><u>` : annullare l'ultima operazione fatta
- `<Esc></>patter_da_cercare<Enter>` : cercare uno specifico pattern nel file (premere `<n>`/`<N>` per navigare tra le occorrenze successive/precedenti)
- `<Esc><:>%s/espressione_regolare/sostituzione/g<Enter>` : eseguire una sostituzione tramite espressione regolare nell'intero file
- `<Esc><Z><Z>|<Esc><:>x<Enter>|<Esc><:>wq<Enter>` : salvare (scrivere) il file per poi uscire
- `<Esc><:>q!<Enter>` : uscire senza salvare

## Links
- More information: https://neovim.io/
