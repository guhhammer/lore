# coredumpctl

Pobieraj i przetwarzaj zapisane zrzuty pamięci i metadane.

## Usage
- `coredumpctl list` : wyświetl wszystkie zapisane zrzuty pamięci
- `coredumpctl list program` : wyświetl zapisane zrzuty pamięci podanego programu
- `coredumpctl info pid` : wyświetl informacje o zrzutach pamięci programu o podanym PID
- `coredumpctl debug program` : wywołaj debugger używając ostatniego zrzutu pamięci programu
- `coredumpctl --output ścieżka/do/pliku dump program` : wyodrębnij ostatni zrzut pamięci programu do pliku

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/coredumpctl.html
