# iexpress

자동 압축 해제 실행 파일(`.exe`) 또는 CAB (`.cab`) 파일을 생성.

## Usage
- `iexpress /n 경로\대상\파일.sed` : 지정된 SED 파일로 자동 압축 해제 ZIP 실행 파일 생성
- `iexpress /n /m 경로\대상\파일.sed` : 최소화된(minimized) 창에서 생성 작업 실행
- `iexpress /n /q:a|u 경로\대상\파일.sed` : 관리자(administrator) 또는 일반 사용자(user) 권한을 가정해, 조용한(quiet) 모드로 생성 작업 실행
- `iexpress /r:a|n|s 경로\대상\파일.sed` : 설치 완료 후 다시 시작(restart) 동작 설정 (ask: 확인, never: 안 함, always: 항상)

## Links
- More information: https://ss64.com/nt/iexpress.html
