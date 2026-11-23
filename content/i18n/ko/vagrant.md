# vagrant

경량, 재생 가능, 휴대 가능한 개발 환경 관리 도구.

## Usage
- `vagrant init` : 현재 디렉토리에 기본 Vagrant 박스로 Vagrantfile 생성
- `vagrant init ubuntu/focal64` : hashiCorp Atlas에서 Ubuntu 20.04 (Focal Fossa) 박스로 Vagrantfile 생성
- `vagrant up` : vagrant 환경 시작 및 프로비저닝
- `vagrant suspend` : 머신 일시 중지
- `vagrant halt` : 머신 중지
- `vagrant ssh` : SSH를 통해 머신에 연결
- `vagrant ssh-config` : 실행 중인 Vagrant 머신의 SSH 구성 파일 출력
- `vagrant box list` : 모든 로컬 박스 나열

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli
