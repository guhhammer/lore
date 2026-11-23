# kubectl wait

Espera a que los recursos alcancen un estado determinado.

## Usage
- `kubectl wait --for condition=available deployment/nombre_del_despliegue` : espera a que un despliegue esté disponible
- `kubectl wait --for condition=ready po -l etiqueta_clave=etiqueta_valor` : espera a que todos los pods con una determinada etiqueta (l) estén listos
- `kubectl wait --for delete po nombre_del_pod` : espera a que se elimine un pod
- `kubectl wait --for condition=complete job/nombre_del_trabajo --timeout 120s` : espera a que se complete un trabajo, en un plazo de 120 segundos (si la condición no se cumple a tiempo, el estado de salida será fallido)

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_wait/
