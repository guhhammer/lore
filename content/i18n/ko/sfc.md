# sfc

Windows 시스템 파일의 무결성을 검사합니다.

## Usage
- `sfc /scannow` : 모든 시스템 파일을 검사하고 가능하면 문제 수정
- `sfc /verifyonly` : 모든 시스템 파일을 검사하고 문제 수정 시도 없음
- `sfc /scanfile=경로\대상\파일` : 특정 파일을 검사하고 가능하면 문제 수정
- `sfc /verifyfile=경로\대상\파일` : 특정 파일을 검사하고 문제 수정 시도 없음
- `sfc /offbootdir=경로\대상\폴더` : 오프라인 복구 시 부팅 디렉터리 지정
- `sfc /offwindir=경로\대상\폴더` : 오프라인 복구 시 Windows 디렉터리 지정
- `sfc` : 도움말 표시
