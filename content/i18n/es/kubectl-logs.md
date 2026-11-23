# kubectl logs

Muestra los registros de los contenedores de un pod.

## Usage
- `kubectl logs nombre_del_pod` : muestra los registros de un pod de un contenedor
- `kubectl logs -c nombre_del_contenedor nombre_del_contenedor` : muestra los registros de un contenedor especificado en un pod
- `kubectl logs --all-containers=true nombre_del_contenedor` : muestra los registros de todos los contenedores de un pod
- `kubectl logs -f nombre_del_pod` : transmite los registros del pod
- `kubectl logs --since tiempo_relativo nombre_del_pod` : muestra los registros de pods más recientes dado un tiempo relativo como `10s`, `5m` o `1h`
- `kubectl logs --tail 10 nombre_del_pod` : muestra los 10 registros más recientes de un pod
- `kubectl logs deploy/nombre_del_despliegue` : muestra todos los registros de un pod para un despliegue determinado

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_logs/
