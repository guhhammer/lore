# git commit-graph

Tulis dan verifikasi file grafik komit Git.

## Usage
- `git commit-graph write` : tulis file grafik komit untuk komit yang dikemas di dalam direktori `.git` pada lokal repositori
- `git show-ref -s | git commit-graph write --stdin-commits` : tulis file grafik komit yang berisi semua komit yang dapat dijangkau
- `git rev-parse HEAD | git commit-graph write --stdin-commits --append` : tulis file grafik komit yang berisi semua komit dalam file grafik komit saat ini beserta yang dapat dijangkau dari `HEAD`

## Links
- More information: https://git-scm.com/docs/git-commit-graph
