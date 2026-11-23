# bcdboot

부트 파일을 구성하거나 복구.

## Usage
- `bcdboot C:\Windows` : 소스 Windows 폴더의 BCD 파일을 사용하여 시스템 파티션 초기화
- `bcdboot C:\Windows /v` : verbose 모드 활성화
- `bcdboot C:\Windows /s S:` : 시스템 파티션의 볼륨 문자 지정
- `bcdboot C:\Windows /l en-us` : locale 지정
- `bcdboot C:\Windows /s S: /f UEFI|BIOS|ALL` : 지정된 볼륨으로 부트 파일을 복사할 때 firmware 유형 지정
