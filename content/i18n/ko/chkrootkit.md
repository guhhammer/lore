# chkrootkit

시스템에서 루트킷을 검사.

## Usage
- `chkrootkit -q` : 조용한(quiet) 모드를 활성화하여 일반 테스트 결과 출력을 생략
- `chkrootkit -x` : 전문가(expert) 모드를 활성화하여 추가 출력을 표시
- `chkrootkit -d` : 디버그(debug) 모드를 활성화하여 모든 출력을 표시
- `chkrootkit -e "경로/대상/파일"` : 일부 테스트에서 제외할(excluded) 파일을 지정
- `chkrootkit -r 경로/대상/디렉터리` : 테스트를 위한 루트(root) 디렉터리를 지정 (예. 마운트된 `ext` 드라이브)
- `chkrootkit -n` : NFS로 마운트된(nfs-mounted) 디렉터리를 무시
- `chkrootkit -T 파일_시스템_유형` : 테스트(Tests)를 실행하며 특정 파일 시스템 유형을 무시
- `chkrootkit -l` : 사용 가능한 테스트 목록(list)을 생성

## Links
- More information: https://manned.org/chkrootkit
