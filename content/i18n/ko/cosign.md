# cosign

OCI 레지스트리의 컨테이너 서명, 검증 및 저장.

## Usage
- `cosign generate-key-pair` : 키의 쌍을 생성
- `cosign sign --key cosign.key 이미지` : 컨테이너에 서명하고 레지스트리에 서명을 저장
- `cosign sign --key k8s://네임스페이스/키 이미지` : kubernetes 비밀에 저장된 키 쌍을 사용하여 컨테이너 이미지에 서명
- `cosign sign-blob --key cosign.key 경로/대상/파일` : 로컬 키 쌍 파일로 blob에 서명
- `cosign verify --key cosign.pub 이미지` : 공개 키에 대해 컨테이너를 확인
- `cosign dockerfile verify -key cosign.pub 경로/대상/Dockerfile` : dockerfile의 공개 키로 이미지를 확인
- `cosign verify --key k8s://네임스페이스/key 이미지` : kubernetes 비밀에 저장된 공개 키로 이미지를 확인
- `cosign copy example.com/src:latest example.com/dest:latest` : 컨테이너 이미지와 해당 서명을 복사

## Links
- More information: https://github.com/sigstore/cosign/blob/main/doc/cosign.md
