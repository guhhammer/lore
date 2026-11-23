# disown

Zorg ervoor dat subprocessen kunnen blijven bestaan nadat de shell waaraan ze gekoppeld zijn, is afgesloten.

## Usage
- `disown` : ontkoppel de huidige job
- `disown %job_number` : ontkoppel een specifieke job (voer `jobs` uit om het jobnummer te vinden)
- `disown -a` : ontkoppel alle jobs (alleen Bash)
- `disown -h %job_number` : behoud de job (ontkoppel deze niet), maar markeer deze zodat er geen toekomstige `SIGHUP` wordt ontvangen bij het afsluiten van de shell (alleen Bash)

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-disown
