# vagrant destroy

게스트 머신을 중지하고 관련된 모든 리소스를 삭제. 설치된 box는 그대로 유지됨.

## Usage
- `vagrant destroy` : 현재 실행 중인 machine 삭제
- `vagrant destroy 이름|id` : 이름 또는 ID를 지정하여 machine 삭제
- `vagrant destroy -f` : 확인 없이 machine 강제 삭제
- `vagrant destroy -g` : machine을 강제 종료하지 않고 정상 종료 후 삭제(gracefully)

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/destroy
