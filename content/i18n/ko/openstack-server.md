# openstack server

OpenStack 가상 머신을 관리하는 명령어. OpenStack Compute 서비스(OpenStack Nova)는 클라우드 컴퓨팅 리소스를 생성 및 관리합니다.

## Usage
- `openstack server list` : 서버 목록 출력
- `openstack server start 인스턴스_아이디1 인스턴스_아이디2 ...` : 서버 시작
- `openstack server stop 인스턴스_아이디1 인스턴스_아이디2 ...` : 서버 중지
- `openstack server create --image 이미지_아이디 --flavor flavor_아이디 --network 네트워크_아이디 --wait 서버_이름` : 새로운 서버 생성
- `openstack server delete 인스턴스_아이디1 인스턴스_아이디2 ...` : 서버 삭제
- `openstack server migrate --live 호스트명 --shared-migration|--block-migration --wait 인스턴스_아이디` : 서버를 다른 호스트로 라이브 마이그레이션
- `openstack server reboot --soft|--hard --wait 인스턴스_아이디` : 서버 소프트/하드 재부팅
