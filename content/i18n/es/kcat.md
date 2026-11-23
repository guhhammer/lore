# kcat

Productor Apache Kafka y herramienta de consumo.

## Usage
- `kcat -C -t tema -b intermediarios` : consume mensajes empezando por el corrimiento (offset) más nuevo
- `kcat -C -t tema -b intermediarios -o beginning -e` : consume mensajes comenzando con el offset más antiguo y sale después de recibir el último mensaje
- `kcat -G id_de_grupo tema -b intermediarios` : consume mensajes como grupo de consumidores de Kafka
- `echo mensaje | kcat -P -t tema -b intermediarios` : publica el mensaje leyendo de `stdin`
- `kcat -P -t tema -b intermediarios ruta/al/archivo` : publica mensajes leyendo desde un archivo
- `kcat -L -b intermediarios` : lista metadatos para todos los temas e intermediarios
- `kcat -L -t tema -b intermediarios` : lista metadatos para un tema específico
- `kcat -Q -t tema:partición:marca_de_tiempo_unix -b intermediarios` : obtiene el offset de un tema/partición para un punto específico en el tiempo

## Links
- More information: https://manned.org/kcat
