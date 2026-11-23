# prowler kubernetes

Kubernetes 클러스터의 보안 모범 사례와 구성 설정을 점검.

## Usage
- `prowler kubernetes` : 기본 kubeconfig를 사용해 기본 보안 검사 실행
- `prowler kubernetes --kubeconfig-file 경로/대상/kubeconfig` : 사용자 지정 kubeconfig 파일을 지정하여 검사
- `prowler kubernetes --context 자신의_컨텍스트` : 지정한 Kubernetes 컨텍스트에 대해 검사
- `prowler kubernetes --namespaces default kube-system` : 지정한 namespace만 검사
- `prowler kubernetes -s ietcd|apiserver|...` : 지정한 Kubernetes 서비스에 대해서만 보안 검사 실행
- `prowler kubernetes -c etcd_암호화` : 지정한 Kubernetes 보안 검사 실행
- `prowler kubernetes -e etcd_암호화 --exclude-services ietcd|apiserver|...` : 지정한 보안 검사 또는 서비스를 제외하고 실행

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
