# docker buildx create

Cria uma nova instância de builder.

## Usage
- `docker buildx create` : cria uma nova instância de builder usando o contexto Docker padrão
- `docker buildx create --name nome_do_builder` : cria uma nova instância de builder com um nome específico
- `docker buildx create --name nome_do_builder --use` : cria uma nova instância de builder e a define imediatamente como o builder ativo atual
- `docker buildx create --driver docker-container|kubernetes|remote|...` : cria uma nova instância de builder usando um driver específico (padrão: `docker`)
- `docker buildx create --platform linux/amd64,linux/arm64,...` : cria uma nova instância de builder com plataformas suportadas específicas
- `docker buildx create --name nome_do_builder --append contexto|endpoint` : adiciona um novo nó a um builder existente
- `docker buildx create --buildkitd-flags "--debug --debugaddr 0.0.0.0:6666"` : cria uma nova instância de builder com flags específicas do daemon BuildKit
- `docker buildx create --name nome_do_builder --bootstrap` : cria uma nova instância de builder e a inicia imediatamente

## Links
- More information: https://docs.docker.com/reference/cli/docker/buildx/create/
