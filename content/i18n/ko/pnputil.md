# pnputil

Windows에서 드라이버 저장소 및 드라이버 패키지를 관리. 관리자 권한의 명령 프롬프트가 필요.

## Usage
- `pnputil /enum-drivers` : 설치된 모든 드라이버 패키지 목록 표시
- `pnputil /add-driver 경로\대상\드라이버.inf` : INF 파일로부터 드라이버 패키지 설치
- `pnputil /add-driver 경로\대상\폴더\*.inf /subdirs` : 폴더 내 모든 INF 파일 추가 및 설치 (하위 디렉터리 포함)
- `pnputil /delete-driver oemXX.inf` : 게시된 이름을 사용하여 드라이버 패키지 삭제
- `pnputil /add-driver oemXX.inf` : 게시된 이름을 사용하여 드라이버 패키지 설치
- `pnputil /export-driver * 경로\대상\폴더` : 모든 드라이버 패키지를 폴더로 내보내기
- `pnputil /enum-devices` : 모든 장치 목록 표시
