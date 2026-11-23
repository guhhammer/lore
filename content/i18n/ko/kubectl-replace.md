# kubectl replace

파일 또는 `stdin`을 통해 리소스를 교체.

## Usage
- `kubectl replace -f 경로/대상/파일.yml` : 리소스 정의 파일을 사용하여 리소스 교체
- `kubectl replace -f -` : `stdin`으로 전달된 입력을 사용하여 리소스 교체
- `kubectl replace -f 경로/대상/파일.yml --force` : 강제로 교체: 리소스를 삭제한 후 다시 생성

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_replace/
