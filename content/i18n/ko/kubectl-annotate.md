# kubectl annotate

Kubernetes 리소스에 annotation 추가 및 관리.

## Usage
- `kubectl annotate po pod_name 키=값` : pod에 annotation 추가
- `kubectl annotate po pod_name 키=값 --overwrite` : 기존 annotation 값을 덮어씌워 업데이트
- `kubectl annotate po 키=값 -l 라벨_키=라벨_값` : 특정 라벨 선택자와 일치하는 네임스페이스 내 모든 Pod에 annotation 추가
- `kubectl annotate po pod_이름 --list` : pod에 설정된 모든 annotation 목록 표시
- `kubectl annotate po pod_이름 키-` : pod에서 annotation 제거

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_annotate/
