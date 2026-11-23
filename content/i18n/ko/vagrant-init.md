# vagrant init

`Vagrantfile`을 생성하여 현재 디렉터리에 Vagrant 환경을 초기화.

## Usage
- `vagrant init` : `Vagrantfile` 생성
- `vagrant init -m` : 안내 주석 없이 최소 구성의 `Vagrantfile` 생성
- `vagrant init box_이름 box_주소` : box 이름과 URL을 지정하여 생성
- `vagrant init --box-version 버전 box_이름` : 특정 box 버전을 지정하여 `Vagrantfile` 생성
- `vagrant init -o -` : `Vagrantfile`을 `stdout`으로 출력
- `vagrant init -f` : 기존 `Vagrantfile` 덮어쓰기
- `vagrant init --template 경로/대상/파일.erb` : 사용자 지정 ERB 템플릿을 사용하여 `Vagrantfile` 생성

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/init
