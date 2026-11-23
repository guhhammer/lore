# crictl

CRI 호환 컨테이너 런타임을 위한 커멘드라인.

## Usage
- `crictl pods` : 모든 Kubernetes 파드 나열 (준비 및 준비되지 않음)
- `crictl ps -a` : 모든 컨테이너 나열 (실행 중 및 종료)
- `crictl images` : 모든 이미지 나열
- `crictl inspect 컨테이너_아이디1 컨테이너_아이디2 ...` : 특정 컨테이너들 정보 표시
- `crictl exec -it 컨테이너_아이디 sh` : 실행 중인 컨테이너 내에서 특정 셸을 열기
- `crictl pull 이미지:태그` : 레지스트리에서 특정 이미지를 가져옴
- `crictl logs -f 컨테이너_아이디` : 특정 컨테이너의 로그를 출력하고 추적(follow)
- `crictl rmi 이미지_아이디1 이미지_아이디2 ...` : 하나 이상의 이미지를 제거

## Links
- More information: https://github.com/kubernetes-sigs/cri-tools/blob/master/docs/crictl.md
