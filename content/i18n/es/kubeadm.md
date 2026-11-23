# kubeadm

Interfaz de línea de comandos para crear y gestionar clusters Kubernetes.

## Usage
- `kubeadm init` : crea un plano de control de Kubernetes
- `kubeadm join --token token` : arranca un nodo trabajador de Kubernetes y lo une a un clúster
- `kubeadm token create --ttl 12h0m0s` : crea un nuevo token de arranque con un TTL de 12 horas
- `kubeadm upgrade plan` : comprueba si el clúster Kubernetes es actualizable y qué versiones están disponibles
- `kubeadm upgrade apply versión` : actualiza el clúster Kubernetes a la versión especificada
- `kubeadm config view` : observa el ConfigMap de kubeadm que contiene la configuración del clúster
- `kubeadm reset` : revierte los cambios realizados en el host por `kubeadm init` o `kubeadm join`

## Links
- More information: https://kubernetes.io/docs/reference/setup-tools/kubeadm/
