# go install

import 경로로 지정된 패키지를 컴파일하고 설치.

## Usage
- `go install` : 현재 패키지를 컴파일하고 설치
- `go install 경로/대상/패키지` : 특정 로컬 패키지를 컴파일하고 설치
- `go install golang.org/x/tools/gopls@최신` : 현재 디렉토리의 `go.mod`를 무시하고 프로그램의 최신 버전 설치
- `go install golang.org/x/tools/gopls` : 현재 디렉토리의 `go.mod`에 의해 선택된 버전으로 프로그램 설치

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Compile_and_install_packages_and_dependencies
