# kubectl

Kubernetes 클러스터에서 명령을 실행하기 위한 명령줄 인터페이스. `run`과 같은 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `kubectl get pods|service|deployment|ingress|... -o wide` : 리소스에 대한 정보를 자세히 나열
- `kubectl label pods 이름 unhealthy=true` : 지정된 포드에 'unhealthy' 레이블과 'true' 값을 추가
- `kubectl get all` : 다양한 유형의 모든 리소스 나열
- `kubectl top pods|nodes` : 노드 또는 포드의 리소스(CPU/메모리/스토리지) 사용량 표시
- `kubectl cluster-info` : 마스터 및 클러스터 서비스의 주소 출력
- `kubectl explain pods.spec.containers` : 특정 필드에 대한 설명 표시
- `kubectl logs 포드_이름` : 포드 또는 지정된 리소스의 컨테이너 로그 출력
- `kubectl exec 포드_이름 -- ls /` : 기존 포드에서 명령 실행

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/
