# iesetup

사용자 지정 Internet Explorer Wizard (`iewzd`) 설치 프로그램을 생성. Internet Explorer Administration Kit (IEAK)의 일부. 참고: 사용하는 IEAK 버전에 따라 `iesetup` 대신 `Ie11setup`(IE 11)과 같은 실행 파일을 사용.

## Usage
- `tldr -p windows iewzd` : `iewzd` 문서 보기
- `iesetup /c:"경로\대상\iewzd.exe"` : `iewzd` 실행 파일 위치 지정해, 사용자 지정 Internet Explorer Wizard 실행 파일 생성
- `iesetup /c:"경로\대상\iewzd.exe iewzd_args"` : 사용자 지정 설치 프로그램에 `iewzd` 프로그램 인수 적용
- `iesetup /q:a|u /c:"경로\대상\iewzd.exe iewzd_args"` : 관리자(administrator) 또는 일반 사용자(user) 권한을 가정해, 조용한(quiet) 모드로 빌드 실행
- `iesetup /r:a|n|s /c:"경로\대상\iewzd.exe iewzd_args"` : 설치 완료 후 다시 시작(restart) 동작 설정 (ask: 확인, never: 안 함, always: 항상)
