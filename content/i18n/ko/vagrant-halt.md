# vagrant halt

Vagrant가 관리하는 실행 중인 machine 종료.

## Usage
- `vagrant halt` : 현재 실행 중인 Vagrant machine을 정상 종료 (gracefully)
- `vagrant halt ID_또는_이름` : ID 또는 이름을 지정하여 특정 머신을 정상 종료
- `vagrant halt -f` : 현재 실행 중인 machine을 강제 종료 (동일한 Vagrant 환경에 여러 machine이 존재하면 함께 종료될 수 있음)
- `vagrant halt -f ID_또는_이름` : ID 또는 이름을 지정하여 특정 머신을 강제 종료

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/halt
