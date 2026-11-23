# evil-winrm

침투 테스트를 위한 WinRM(Windows 원격 관리 쉘). 연결되면, 대상 호스트에 PowerShell 프롬프트가 표시됨.

## Usage
- `evil-winrm -i 아이피 -u 사용자 -p 비밀번호` : 호스트에 연결
- `evil-winrm -i 아이피 -u 사용자 -H nt_hash` : 비밀번호 해시를 전달하여 호스트에 연결
- `evil-winrm -i 아이피 -u 사용자 -p 비밀번호 -s 경로/대상/스크립트 -e 경로/대상/실행파일` : 스크립트 및 실행 파일에 대한 디렉터리를 지정하여 호스트에 연결
- `evil-winrm -i 아이피 -u 사용자 -p 비밀번호 -S -c 경로/대상/공개키 -k 경로/대상/개인키` : SSL을 사용하여 호스트에 연결
- `upload 경로/대상/로컬/파일 경로/대상/원격/파일` : [대화형] 호스트에 파일 업로드
- `menu` : [대화형] 로드된 모든 PowerShell 함수를 나열
- `스크립트.ps1` : [대화형] `--scripts` 디렉터리에서 PowerShell 스크립트를 로드
- `Invoke-Binary 바이너리.exe` : [대화형] `--executables` 디렉터리에서 호스트의 바이너리를 호출

## Links
- More information: https://github.com/Hackplayers/evil-winrm
