# msiexec

MSI 및 MSP 패키지 파일을 사용하여 Windows 프로그램 설치, 업데이트, 수리 또는 제거.

## Usage
- `msiexec /package 경로\대상\파일.msi` : MSI 패키지에서 프로그램 설치
- `msiexec /package https://example.com/installer.msi` : 웹사이트에서 MSI 패키지 설치
- `msiexec /update 경로\대상\파일.msp` : MSP 패치 파일 설치
- `msiexec /uninstall 경로\대상\파일` : 프로그램 또는 패치 제거 (각각의 MSI 또는 MSP 파일 사용)
