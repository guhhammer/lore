# git rev-list

Elenca le revisioni (commit) in ordine cronologico inverso.

## Usage
- `git rev-list HEAD` : mostra tutti i commit del ramo corrente
- `git rev-list --since='2019-12-01 00:00:00' nome_ramo` : mostra i commit più recenti di una certa data, su uno specifico ramo
- `git rev-list --merges commit` : mostra tutti i commit di unione (merge commit) associati a uno specifico commit

## Links
- More information: https://git-scm.com/docs/git-rev-list
