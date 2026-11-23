# docker buildx prune

Удалять кэш сборки.

## Usage
- `docker buildx prune` : удалить кэш сборки для текущего активного сборщика
- `docker buildx prune --filter "type=source.local"` : удалить записи кэша на основе определённого фильтра
- `docker buildx prune --max-used-space 128mb` : удалить наименее используемые записи кэша до достижения определённого размера кэша
- `docker buildx prune --reserved-space 2gb` : удалить наименее используемые записи кэша до достижения определённого объёма свободного места на диске

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/prune/
