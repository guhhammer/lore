# az container

Azure Container Instances 관리. `azure-cli`의 일부 (`az`).

## Usage
- `az container create -g 리소스_그룹 -n 이름 --image 이미지_이름 -os windows|linux --cpu cpu_코어_개수 --memory memory_in_GB` : 컨테이너 그룹에 컨테이너 생성
- `az container exec -g 리소스_그룹 -n 컨테이너_그룹_이름 --exec-command "명령어"` : 실행 중인 컨테이너에서 명령어 실행
- `az container logs -n 이름 -g 리소스_그룹` : 컨테이너 그룹 안 컨테이너 로그 확인
- `az container show -n 이름 -g 리소스_그룹` : 컨테이너 그룹 상세 정보 조회
- `az container start -n 이름 -g 리소스_그룹` : 컨테이너 그룹 내 모든 컨테이너 시작
- `az container stop -n 이름 -g 리소스_그룹` : 컨테이너 그룹 내 모든 컨테이너 중지
- `az container delete -n 이름 -g 리소스_그룹` : 컨테이너 그룹 삭제

## Links
- More information: https://learn.microsoft.com/cli/azure/container
