# flux uninstall

Flux와 사용자 지정 리소스 정의(CRD)를 제거.

## Usage
- `flux uninstall` : flux 컴포넌트, 사용자 지정 리소스 및 네임스페이스 제거
- `flux uninstall -s` : 확인 메시지 없이 Flux 제거
- `flux uninstall -n 네임스페이스` : 지정한 네임스페이스에서 Flux 제거
- `flux uninstall --keep-namespace` : 네임스페이스는 유지하고 Flux 제거
- `flux uninstall --dry-run` : flux를 실제로 제거하지 않고 제거 과정 미리 확인

## Links
- More information: https://fluxcd.io/flux/cmd/flux_uninstall/
