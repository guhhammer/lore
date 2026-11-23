# mkfifo

Maak FIFOs (benoemde pipes).

## Usage
- `mkfifo pad/naar/pipe` : maak een benoemde pipe op een opgegeven pad
- `echo "Hello World" >pad/naar/pipe &` : stuur data naar een benoemde pipe en stuur het commando naar de achtergrond
- `cat pad/naar/pipe` : ontvang data van een benoemde pipe
- `mkfifo pad/naar/pipe; script -f pad/naar/pipe` : deel je terminalsessie in real-time
