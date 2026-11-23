# tlmgr restore

`tlmgr backup`으로 생성된 패키지 백업 복원. 기본 백업 디렉토리는 `backupdir` 옵션에 의해 지정되며, `tlmgr option`으로 확인할 수 있습니다.

## Usage
- `tlmgr restore` : 모든 패키지에 대한 사용 가능한 모든 백업 리비전 나열
- `tlmgr restore 패키지` : 특정 패키지에 대한 사용 가능한 모든 백업 리비전 나열
- `tlmgr restore 패키지 리비전` : 특정 패키지의 특정 리비전 복원
- `tlmgr restore --all` : 백업된 모든 패키지의 최신 리비전 복원
- `tlmgr restore 패키지 리비전 --backupdir 경로/대상/백업_디렉토리` : 사용자 지정 백업 디렉토리에서 패키지 복원
- `tlmgr restore --dry-run 패키지 리비전` : 수행된 모든 작업을 출력하고 실제로 수행하지는 않음 (드라이런)

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#restore
