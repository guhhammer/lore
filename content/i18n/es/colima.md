# colima

Contenedores en tiempo de ejecución para macOS y Linux con una configuración mínima.

## Usage
- `colima start` : inicia el daemon en segundo plano
- `colima start --edit` : crea un archivo de configuración y lo usa
- `colima start --runtime containerd` : inicia y configura containerd (instala `nerdctl` para usar containerd a través de `nerdctl`)
- `colima start --kubernetes` : inicia con Kubernetes (se requiere `kubectl`)
- `colima start --cpu número --memory memoria --disk espacio_de_almacenamiento` : personaliza el recuento de CPU, memoria RAM y espacio en disco (en GiB)
- `colima start` : usa Docker a través de Colima (se requiere Docker)
- `colima list` : lista contenedores con su información y estado
- `colima status` : muestra estado de tiempo de ejecución

## Links
- More information: https://github.com/abiosoft/colima
