# jmtpfs

MTP 장치에 액세스하기 위한 FUSE 기반 파일 시스템.

## Usage
- `jmtpfs 경로/대상/폴더` : MTP 장치를 디렉토리에 마운트
- `jmtpfs -o allow_other,auto_unmount 경로/대상/폴더` : 마운트 옵션 설정
- `jmtpfs -l` : 사용 가능한 MTP 장치 나열
- `jmtpfs -device=버스_id,장치_id 경로/대상/폴더` : 여러 장치가 있는 경우 특정 장치 마운트
- `fusermount -u 경로/대상/폴더` : MTP 장치 마운트 해제

## Links
- More information: https://manned.org/jmtpfs
