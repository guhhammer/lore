# cmd

Windows 명령 인터프리터.

## Usage
- `cmd` : 대화형 셸 세션 시작
- `cmd /c echo cmd가 실행중` : 특정 commands 실행
- `cmd 경로\대상\script.bat` : 특정 스크립트 실행
- `cmd /k echo cmd가 실행중` : 특정 명령을 실행한 후 대화형 셸로 진입
- `cmd /q` : 명령 출력에서 `echo`가 비활성화된 대화형 셸 세션 시작
- `cmd /v:on|off` : 지연된 variable 확장이 활성화 또는 비활성화된 대화형 셸 세션 시작
- `cmd /e:on|off` : 명령 extensions가 활성화 또는 비활성화된 대화형 셸 세션 시작
- `cmd /u` : unicode 인코딩을 사용하는 대화형 셸 세션 시작
