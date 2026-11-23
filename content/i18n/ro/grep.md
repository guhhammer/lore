# grep

Caută tipare în fișiere folosind expresii regulate (`regex`).

## Usage
- `grep "tipar_de_căutare" cale/către/fișier1 cale/către/fișier2 ...` : caută un tipar într-un set de fișiere
- `grep -F "șir_exact" cale/către/fișier` : caută un șir de caractere exact (dezactivează `regex`-ul)
- `grep -rI "tipar_de_căutare" cale/către/director` : caută un tipar recursiv în toate fișierele dintr-un folder, ignorând fișierele binare
- `grep --context|--before-context|--after-context 3 "tipar_de_căutare" cale/către/fișier` : afișează 3 linii de Context (în jur, îBainte sau After de fiecare potrivire)
- `grep -Hn --color=always "tipar_de_căutare" cale/către/fișier` : afișează numele fișierului și numărul liniei pentru fiecare potrivire, cu ieșire colorată
- `grep -o "tipar_de_căutare" cale/către/fișier` : afișează doar textul potrivit
- `cat cale/către/fișier | grep -v "tipar_de_căutare"` : citește datele din `stdin` și nu afișează liniile care se potrivesc cu tiparul
- `grep -Ei "tipar_de_căutare" cale/către/fișier` : folosește `regex` extins (suportă `?`, `+`, `{}`, `()` și `|`), în mod insensibil la majuscule

## Links
- More information: https://www.gnu.org/software/grep/manual/grep.html
