# hyperfine

Инструмент для замера производительности.

## Usage
- `hyperfine 'make'` : выполнить базовый замер, проведя не менее 10 запусков
- `hyperfine 'make target1' 'make target2'` : выполнить сравнительный замер
- `hyperfine -m 7 'make'` : изменить минимальное количество запусков
- `hyperfine -w 5 'make'` : выполнить замер с предварительным прогревом (warmup)
- `hyperfine -p 'make clean' 'make'` : выполнить команду перед каждым запуском замера (например, для очистки кэша)
- `hyperfine -p 'make clean' -P количество_потоков 1 10 'make --jobs {количество_потоков}'` : выполнить замер, изменяя один параметр при каждом запуске

## Links
- More information: https://manned.org/hyperfine
