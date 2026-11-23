# vagrant plugin

Vagrant 플러그인을 관리.

## Usage
- `vagrant plugin list` : 현재 설치된 모든 플러그인 목록 표시
- `vagrant plugin install vagrant_vbguest` : 원격 저장소에서 플러그인 설치 (일반적으로 RubyGems)
- `vagrant plugin install 경로/대상/자신의_플러그인.gem` : 로컬 파일에서 플러그인 설치
- `vagrant plugin update` : 설치된 모든 플러그인을 최신 버전으로 업데이트
- `vagrant plugin update vagrant_vbguest` : 지정한 플러그인을 최신 버전으로 업데이트
- `vagrant plugin uninstall vagrant_vbguest` : 지정한 플러그인 제거

## Links
- More information: https://developer.hashicorp.com/vagrant/docs/cli/plugin
