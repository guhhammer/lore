# gau

Obtén todas las URLs: obtén las URLs conocidas de Open Threat Exchange de AlienVault, Wayback Machine y Common Crawl para cualquier dominio.

## Usage
- `gau example.com` : obtén todas las URLs de un dominio de Open Threat Exchange de AlienVault, Wayback Machine, Common Crawl y URLScan
- `gau dominio1 dominio2 ...` : obtén URLs de varios dominios
- `gau <ruta/a/dominios.txt --threads 4` : obtén todas las URLs de varios dominios en un archivo de entrada, ejecutando varios subprocesos
- `gau example.com --o ruta/a/urls_encontradas.txt` : escribe los resultados en un archivo
- `gau --providers wayback|commoncrawl|otx|urlscan example.com` : busca las URLs de un solo proveedor específico
- `gau --providers wayback,otx,... example.com` : busca las URLs de varios proveedores
- `gau --from AAAAMM --to YYYYMM example.com` : busca las URLs dentro de un intervalo de fechas específico

## Links
- More information: https://github.com/lc/gau#usage
