# php

Interface de linha de comando do PHP.

## Usage
- `php caminho/para/arquivo` : analisa e executa um script PHP
- `php -l caminho/para/arquivo` : verifica a sintaxe (lint) de um script PHP
- `php -a` : executa o PHP de forma interativa
- `php -r "código"` : executa código PHP (não use as tags `<? ?>`; escape aspas duplas com barra invertida)
- `php -S servidor:porta` : inicia o servidor web embutido do PHP no diretório atual
- `php -m` : lista as extensões do PHP instaladas
- `php -i` : exibe informações sobre a configuração atual do PHP
- `php --rf nome_da_função` : exibe informações sobre uma função específica

## Links
- More information: https://www.php.net/manual/en/features.commandline.options.php
