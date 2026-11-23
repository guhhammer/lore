# aplay

Konsolowy odtwarzacz dźwięku dla sterownika dźwiękowego ALSA.

## Usage
- `aplay ścieżka/do/pliku` : odtwórz określony plik (częstotliwość próbkowania, ilość bitów, itd. będą określane automatycznie na podstawie formatu)
- `aplay --duration=10 --rate=2500 ścieżka/do/pliku` : odtwórz pierwsze 10 sekund określonego pliku z częstotliwością 2500 Hz
- `aplay --channels=1 --file-type raw --rate=22050 --format=mu_law ścieżka/do/pliku` : odtwórz surowy plik jako plik Mu-Law `.au`, 22050 Hz, mono, 8-bit

## Links
- More information: https://manned.org/aplay
