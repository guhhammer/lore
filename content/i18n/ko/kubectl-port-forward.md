# kubectl port-forward

하나 이상의 로컬 포트를 파드로 포워딩.

## Usage
- `kubectl port-forward po/pod_이름 5000 6000` : 로컬 포트 5000, 6000을 Pod의 5000, 6000 포트로 포워딩
- `kubectl port-forward po/pod_이름 :5000` : 임의의 로컬 포트를 Pod의 5000 포트로 포워딩
- `kubectl port-forward deploy/deployment_이름 5000 6000` : 로컬 포트 5000, 6000을 Deployment의 5000, 6000 포트로 포워딩
- `kubectl port-forward svc/서비스_이름 8443:https` : 로컬 포트 8443을 Service의 https 포트로 포워딩
- `kubectl port-forward po/pod_이름 8888:5000 --address 0.0.0.0` : 모든 네트워크 인터페이스에서 8888 포트를 Pod의 5000 포트로 포워딩
- `kubectl port-forward po/pod_이름 8888:5000 --address localhost,10.19.21.23` : localhost와 특정 IP에서 8888 포트를 Pod의 5000 포트로 포워딩

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_port-forward/
