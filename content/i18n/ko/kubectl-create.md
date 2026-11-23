# kubectl create

파일 또는 `stdin`에서 리소스를 생성.

## Usage
- `kubectl create -f 경로/대상/파일.yml` : 리소스 정의 파일을 사용하여 리소스 생성
- `kubectl create -f -` : `stdin`에서 리소스 생성
- `kubectl create deploy 배포_이름 --image 이미지` : 배포 생성
- `kubectl create deploy 배포_이름 --image 이미지 --replicas 복제본_수` : 복제본과 함께 배포 생성
- `kubectl create svc 서비스_유형 서비스_이름 --tcp 포트:대상_포트` : 서비스 생성
- `kubectl create ns 네임스페이스_이름` : 네임스페이스 생성

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_create/
