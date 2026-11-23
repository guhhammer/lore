# kubectl cp

로컬 파일 시스템과 Pod 내부 컨테이너 간 파일 및 디렉터리 복사.

## Usage
- `kubectl cp 경로/대상/로컬_파일 pod_이름:/경로/대상/Pod_내_파일` : 로컬 파일 또는 디렉터리를 Pod로 복사
- `kubectl cp 경로/대상/로컬_파일 네임스페이스/pod_이름:/경로/대상/Pod_내_파일` : 특정 네임스페이스의 Pod로 로컬 파일 또는 디렉터리 복사
- `kubectl cp 네임스페이스/pod_이름:/경로/대상/Pod_내_파일 경로/대상/로컬_파일` : pod의 파일 또는 디렉터리를 로컬 머신으로 복사
- `kubectl cp 경로/대상/로컬_파일 pod_이름:/경로/대상/Pod_내_파일 -c 컨테이너_이름` : pod의 특정 컨테이너로 파일 또는 디렉터리 복사
- `kubectl cp 경로/대상/로컬_파일 네임스페이스/pod_이름:/경로/대상/Pod_내_파일 --no-preserve` : 소유권 및 권한을 유지하지 않고 Pod로 복사
- `kubectl cp 경로/대상/로컬_파일 pod_이름:/경로/대상/Pod_내_파일 --retries 3` : 실패 시 지정한 횟수만큼 재시도하며 Pod로 복사

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_cp/
