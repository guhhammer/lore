# helm install

Instala un chart de helm.

## Usage
- `helm install nombre nombre_del_repositorio/nombre_del_chart` : instala un chart de helm
- `helm install nombre ruta/al/directorio_de_origen` : instala un chart de helm desde un directorio de chart desempaquetado
- `helm install nombre_del_paquete https://example.com/charts/packagename-1.2.3.tgz` : instala un chart de helm desde una URL
- `helm install nombre_del_repositorio/nombre_del_chart -g` : instala un chart de helm y genera un nombre
- `helm install nombre nombre_del_repositorio/nombre_del_chart --dry-run` : realiza una simulación
- `helm install nombre nombre_del_repositorio/nombre_del_chart --set parámetro1=valor1,parámetro2=valor2` : instala un chart de helm con valores personalizados
- `helm install nombre nombre_del_repositorio/nombre_del_chart -f ruta/a/valores.yaml` : instala un chart de helm pasando un archivo de valores personalizados

## Links
- More information: https://helm.sh/docs/intro/using_helm/#helm-install-installing-a-package
