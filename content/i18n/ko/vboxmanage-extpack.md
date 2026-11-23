# vboxmanage-extpack

Oracle VirtualBox용 확장팩 관리 도구.

## Usage
- `VBoxManage extpack install 경로/대상/파일.vbox-extpack` : virtualBox에 확장팩 설치 (참고: 새 버전을 설치하기 전에 기존 버전을 제거해야 함)
- `VBoxManage extpack install --replace` : 기존 VirtualBox 확장팩 버전 제거
- `VBoxManage extpack uninstall 확장팩_이름` : virtualBox에서 확장팩 제거
- `VBoxManage extpack uninstall --force 확장팩_이름` : 대부분의 제거 거부를 건너뛰고 확장팩 제거
- `VBoxManage extpack cleanup` : 확장팩이 남긴 임시 파일 및 디렉토리 정리

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-extpack
