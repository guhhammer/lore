# iewzd

Internet Explorer Setup Wizard: Internet Explorer 설치를 프로그래밍 방식으로 구성. 사용자 지정 Internet Explorer 및 `iewzd`를 함께 포함하는 `iesetup`과 혼동하지 말 것. 참고: 사용하는 IE 버전에 따라 `iewzd` 대신 `Ie11wzd.exe`, `ie10_setup.exe`, `IE9-Windows6.0-x86-en-us.exe` 등 실행 파일을 사용해야 할 수 있음.

## Usage
- `tldr -p windows iesetup` : `iesetup` 문서 보기
- `iewzd` : internet Explorer Wizard 실행
- `iewzd /norestart|/forcerestart` : internet Explorer 설치 후 컴퓨터를 강제로 다시 시작하거나 다시 시작하지 않음
- `iewzd /no-default` : internet Explorer를 기본 브라우저로 설정하지 않음
- `iewzd /update-no` : internet Explorer 업데이트 확인 안 함
- `iewzd /passive|/quiet` : 수동 모드(Passive, 진행 상황만 표시) 또는 조용한 모드(Quiet, 화면 표시 없이 백그라운드)로 실행
- `iewzd /log 경로\대상\파일` : 설치 로그를 파일에 저장
