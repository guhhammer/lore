# flux build

Flux 매니페스트를 생성.

## Usage
- `flux build kustomization 이름 --path 경로/대상/kustomization` : kustomization 매니페스트 생성
- `flux build kustomization 이름 --path 경로/대상/kustomization >경로/대상/manifests.yaml` : kustomization 매니페스트를 생성하여 파일로 저장
- `flux build kustomization 이름 --path 경로/대상/kustomization | kubectl apply -f -` : kustomization 매니페스트를 생성하고 `kubectl`에 직접 전달하여 적용

## Links
- More information: https://fluxcd.io/flux/cmd/flux_build/
