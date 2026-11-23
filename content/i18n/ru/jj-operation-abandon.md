# jj operation abandon

Отбрасывать историю операций в репозитории `jj`.

## Usage
- `jj op abandon идентификатор_операции` : отбросить конкретную операцию
- `jj op abandon ..идентификатор_операции` : отбросить конкретную операцию и всех её предков
- `jj op abandon идентификатор_операции..@-` : отбросить недавние операции после восстановления к более раннему состоянию

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-operation-abandon
