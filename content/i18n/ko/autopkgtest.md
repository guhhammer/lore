# autopkgtest

Debian 패키지에 대한 테스트 실행.

## Usage
- `autopkgtest -- null` : 현재 디렉터리의 패키지를 빌드하고 모든 테스트를 시스템에서 직접 실행
- `autopkgtest --test-name=테스트_이름 -- null` : 현재 디렉터리의 패키지에 대해 특정 테스트 실행
- `autopkgtest 패키지 -- null` : `apt-get`으로 특정 패키지를 다운로드 및 빌드한 후 모든 테스트 실행
- `autopkgtest -- chroot 경로/대상/새로운/루트` : 새로운 루트 디렉터리를 사용하여 현재 디렉터리의 패키지 테스트
- `autopkgtest -B -- null` : 현재 디렉터리의 패키지를 재빌드하지 않고 테스트

## Links
- More information: https://manned.org/autopkgtest
