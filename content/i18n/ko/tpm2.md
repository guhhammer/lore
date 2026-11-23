# tpm2

하나의 통합 실행 파일을 통해 다양한 tpm2-tools 명령을 실행. `pcrread`, `pcrreset` 등의 일부 하위 명령은 각각 별도의 사용 문서를 제공.

## Usage
- `tpm2 pcrreset 16` : PCR 16 뱅크 초기화
- `tpm2 pcrextend 16:sha1=f1d2d2f924e986ac86fdf7b36c94bcdf32beec15` : PCR 16 sha1 뱅크를 `f1d2d2f924e986ac86fdf7b36c94bcdf32beec15` 해시값으로 확장
- `tpm2 pcrread sha1:16` : PCR 16 sha1 뱅크 읽기
- `tpm2 -V 하위명령어` : 자세한 모드로 하위 명령 실행
- `tpm2 -Q 하위명령어` : `stdout`에 출력하지 않고 하위 명령 실행
- `tpm2 -h` : 도움말 표시

## Links
- More information: https://manned.org/tpm2
