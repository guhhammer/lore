# touch

파일을 생성하고 접근/수정 시간을 설정합니다.

## Usage
- `touch 경로/대상/파일1 경로/대상/파일2 ...` : 특정 파일 생성
- `touch -c -a|m 경로/대상/파일1 경로/대상/파일2 ...` : 파일의 access 또는 modification 시간을 현재 시간으로 설정하고 파일이 없으면 create 하지 않음
- `touch -c -t YYYYMMDDHHMM.SS 경로/대상/파일1 경로/대상/파일2 ...` : 파일의 time을 특정 값으로 설정하고 파일이 없으면 create 하지 않음
- `touch -c -r 경로/대상/참조_파일 경로/대상/파일1 경로/대상/파일2 ...` : 파일의 타임스탬프를 reference 파일의 타임스탬프로 설정하고 파일이 없으면 create 하지 않음
