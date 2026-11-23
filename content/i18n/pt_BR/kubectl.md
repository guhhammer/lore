# kubectl

Linha de comando para executar comando em clusters do Kubernetes. Alguns subcomandos como `run` tem sua própia documentação de uso.

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : lista toda a informação sobre um recurso em detalhes
- `kubectl label pods name unhealthy=true` : atualiza um pod específico com o label 'unhealthy' e o valor 'true'
- `kubectl get all` : lista todos os recursos de diferentes tipos
- `kubectl top pods|nodes` : exibe os usos de recursos (CPU/Memória/Espaço alocado) dos nós ou pods
- `kubectl cluster-info` : exibe os endereços dos serviços do master e do cluster
- `kubectl explain pods.spec.containers` : exibe uma explicação de um campo específico
- `kubectl logs pod_name` : exibe os logs de um container em um pod ou de um recurso específico
- `kubectl exec pod_name -- ls /` : executa um comando em um pod existente

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
