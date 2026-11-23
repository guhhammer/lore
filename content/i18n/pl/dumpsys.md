# dumpsys

Uzyskaj informacje o usługach systemu Android. Ta komenda może być używana tylko poprzez `adb shell`.

## Usage
- `dumpsys` : uzyskaj dane diagnostyczne dla wszystkich usług systemowych
- `dumpsys usługa` : uzyskaj dane diagnostyczne dla określonej usługi systemowej
- `dumpsys -l` : wypisz wszystkie usługi, o których `dumpsys` może podać informacje
- `dumpsys usługa -h` : wypisz argumenty specyficzne dla danej usługi
- `dumpsys --skip usługa` : wyklucz określoną usługę z wyjścia diagnostycznego
- `dumpsys -t 8` : określ czas oczekiwania w sekundach (domyślnie 10s)

## Links
- More information: https://developer.android.com/tools/dumpsys
