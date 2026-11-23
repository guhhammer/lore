# kubectl label

Kubernetes 리소스에 레이블 지정.

## Usage
- `kubectl label pod 포드_이름 키=값` : 포드에 레이블 지정
- `kubectl label --overwrite pod 포드_이름 키=값` : 기존 값을 덮어쓰며 포드 레이블 업데이트
- `kubectl label pods --all 키=값` : 네임스페이스의 모든 포드에 레이블 지정
- `kubectl label -f 포드_정의_파일 키=값` : 포드 정의 파일로 식별된 포드에 레이블 지정
- `kubectl label pod 포드_이름 키-` : 포드에서 레이블 제거

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_label/
