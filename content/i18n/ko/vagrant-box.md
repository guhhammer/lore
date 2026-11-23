# vagrant box

Vagrant box (가상 머신 이미지)를 관리.

## Usage
- `vagrant box list` : 모든 설치된 box 목록 표시
- `vagrant box add hashicorp/bionic64` : 새로운 box 추가
- `vagrant box add 자신의-box https://example.com/my-box.box` : 사용자 지정 URL에서 box 추가
- `vagrant box remove hashicorp/bionic64` : 설치된 box 제거
- `vagrant box update` : 현재 Vagrant 환경에서 사용 중인 모든 box 업데이트
- `vagrant box update --box bento/debian-12` : 지정한 box 업데이트
- `vagrant box outdated` : 현재 사용 중인 box의 새로운 버전이 있는지 확인
- `vagrant box prune` : 설치된 box의 이전 버전 정리

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/box
