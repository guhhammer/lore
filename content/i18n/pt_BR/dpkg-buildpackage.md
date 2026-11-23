# dpkg-buildpackage

Compila pacotes binários e/ou fonte no formato Debian a partir do código-fonte. Normalmente é executado dentro de uma árvore de fontes que contém um diretório `debian/`. Também pode verificar dependências de compilação, gerar arquivos como `.buildinfo` e `.changes`, e assinar o resultado quando aplicável.

## Usage
- `dpkg-buildpackage` : gera pacotes fonte e binário
- `dpkg-buildpackage -b` : gera apenas pacotes binários (sem pacote fonte)
- `dpkg-buildpackage -S` : gera apenas o pacote fonte (sem compilar binários)
- `dpkg-buildpackage -us -uc` : não assina os arquivos `.dsc` e `.changes`
- `dpkg-buildpackage -nc` : não executa `clean` antes de compilar
- `dpkg-buildpackage -rfakeroot` : usa `fakeroot` como comando para obter privilégios de root durante o build
- `dpkg-buildpackage -Tclean` : executa um alvo específico de `debian/rules`

## Links
- More information: https://manned.org/dpkg-buildpackage
