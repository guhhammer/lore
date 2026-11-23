# duc

Uma coleção de ferramentas para indexar, inspecionar e visualizar uso do disco. O duc mantém uma base de dados dos tamanhos acumlados dos diretórios do sistema de arquivos, permitindo buscas nessa base, ou a criação de gráficos elegantes.

## Usage
- `duc index /usr` : indexa o diretório /usr, escrevendo a base de dados para o local default em ~/.duc.db
- `duc ls -Fg /usr/local` : lista todos os arquivos e diretórios dentro do /usr/local, mostrando tamanho relativo dos arquivos em um graph (gráfico)
- `duc ls -Fg -R /usr/local` : lista todos os arquivos e diretórios dentro do /usr/local em uma visão de árvore recursiva
- `duc gui /usr` : inicia uma interface gráfica para o usuário explorar o sistema de arquivos exibindo o gráfico sunburst
- `duc ui /usr` : executa a interface de console ncurses para explorar o sistema de arquivos
- `duc info` : exporta as informações da base de dados
