# az redis

Redis 캐시 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az redis create --location 위치 -n 이름 -g 리소스_그룹 --sku Basic|Premium|Standard --vm-size c0|c1|c2|c3|c4|c5|c6|p1|p2|p3|p4|p5` : 새로운 Redis 캐시 인스턴스를 생성
- `az redis update -n 이름 -g 리소스_그룹 --sku Basic|Premium|Standard --vm-size c0|c1|c2|c3|c4|c5|c6|p1|p2|p3|p4|p5` : redis 캐시 업데이트
- `az redis export --container 컨테이너 --file-format 파일-포맷 -n 이름 --prefix 접두사 -g 리소스_그룹` : redis 캐시에 저장된 데이터 내보내기
- `az redis delete -n 이름 -g 리소스_그룹 -y` : redis 캐시 삭제

## Links
- More information: https://learn.microsoft.com/cli/azure/redis
