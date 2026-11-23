# mv

Mută sau redenumește fișiere și foldere.

## Usage
- `mv cale/către/sursă cale/către/destinație` : redenumește un fișier sau folder atunci când destinația nu este un folder existent
- `mv cale/către/sursă cale/către/folder_existent` : mută un fișier sau folder într-un folder existent
- `mv cale/către/sursă1 cale/către/sursă2 ... cale/către/folder_existent` : mută mai multe fișiere într-un folder existent, păstrând numele fișierelor
- `mv -f cale/către/sursă cale/către/destinație` : nu cere confirmare înainte de a suprascrie fișierele existente
- `mv -i cale/către/sursă cale/către/destinație` : cere confirmare interactiv înainte de a suprascrie fișierele existente, indiferent de permisiuni
- `mv -n cale/către/sursă cale/către/destinație` : nu suprascrie fișierele existente la destinație
- `mv -v cale/către/sursă cale/către/destinație` : mută fișiere în mod detaliat, afișându-le după ce sunt mutate
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t cale/către/folder_destinație` : specifică folderul de destinație pentru a putea folosi instrumente externe care colectează fișiere mutabile

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
