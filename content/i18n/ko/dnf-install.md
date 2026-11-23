# dnf install

Red Hat 계열 배포판에서 패키지 설치.

## Usage
- `sudo dnf in 패키지1 패키지2 ...` : 이름으로 패키지 설치
- `sudo dnf in 경로/대상/파일` : 로컬 파일로부터 패키지 설치
- `sudo dnf in https://example.com/package.rpm` : 인터넷에서 패키지 설치
- `sudo dnf in https://mirrors.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm` : fedora 자유 아닌 패키지 저장소 추가
- `sudo dnf in epel-release` : EPEL(Extra Packages for Enterprise Linux) 저장소 추가
- `sudo dnf in https://rpms.remirepo.net/enterprise/remi-release-8.rpm` : remi's RPM 저장소 추가

## Links
- More information: https://dnf.readthedocs.io/en/latest/command_ref.html#install-examples
