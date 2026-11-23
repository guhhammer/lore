# kubectl attach

이미 실행중인 컨테이너 내부 프로세스에 연결.

## Usage
- `kubectl attach pod_이름` : pod의 출력 확인
- `kubectl attach pod_이름 -c 컨테이너_이름` : 지정한 Pod의 특정 컨테이너 출력 확인
- `kubectl attach rs/replicaset_이름` : replicaSet의 첫 번째 Pod 출력 확인
- `kubectl attach pod_이름 -it` : pod에 대화형 세션 연결
- `kubectl attach pod_이름 -c 컨테이너_이름 -it` : pod의 특정 컨테이너에 대화형 세션 연결

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_attach/
