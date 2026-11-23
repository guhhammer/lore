# ack

Narzędzie wyszukiwania, podobne do `grep`, zoptymalizowane dla programistów.

## Usage
- `ack "wzorzec_wyszukiwania"` : szukaj rekurencyjnie plików zawierających ciąg znaków lub `regex` w bieżącym katalogu
- `ack -i "wzorzec_wyszukiwania"` : szukaj na podstawie wzorca bez uwzględniania wielkości liter
- `ack -o "wzorzec_wyszukiwania"` : szukaj linii zawierających wzorzec, wyświetlając tylko pasujący tekst bez pozostałej zawartości linii
- `ack -t ruby "wzorzec_wyszukiwania"` : ogranicz wyszukiwanie do plików wyłącznie określonego typu
- `ack -t noruby "wzorzec_wyszukiwania"` : wyszukaj z pominięciem plików określonego typu
- `ack -c -h "wzorzec_wyszukiwania"` : policz całkowitą liczbę znalezionych dopasowań
- `ack -c -l "wzorzec_wyszukiwania"` : pokaż nazwy plików i liczbę dopasowań w każdym z nich
- `ack --help-types` : wypisz wszystkie możliwe wartości które mogą być użyte dla `--type`

## Links
- More information: https://beyondgrep.com/documentation/
