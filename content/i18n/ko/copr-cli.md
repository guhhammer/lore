# copr-cli

Fedora-Projects의 copr 인스턴스와 상호작용하여 RPM 빌드 및 배포를 수행하는 도구.

## Usage
- `copr-cli whoami` : 현재 로그인된 사용자 확인
- `copr-cli build 저장소 경로/대상/스펙_파일` : 로컬 spec 파일을 copr에서 빌드
- `copr-cli list-builds 저장소` : 빌드 상태 확인
- `copr-cli buildscm 저장소 --clone-url https://git.example.org/repo --spec 스펙_파일_이름` : 공개 Git 저장소의 spec

## Links
- More information: https://manned.org/copr-cli
