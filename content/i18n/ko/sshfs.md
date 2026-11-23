# sshfs

SSH 기반 파일 시스템 클라이언트.

## Usage
- `sshfs 사용자_명@원격_호스트:원격_디렉터리 마운트_지점` : 원격 디렉터리 마운트
- `umount 마운트_지점` : 원격 디렉터리 마운트 해제
- `sshfs 사용자_명@원격_호스트:원격_디렉터리 -p 2222` : 특정 포트로 서버의 원격 디렉터리 마운트
- `sshfs 사용자_명@원격_호스트:원격_디렉터리 -C` : 압축 사용
- `sshfs -o follow_symlinks 사용자_명@원격_호스트:원격_디렉터리 마운트_지점` : 심볼릭 링크 따라가기

## Links
- More information: https://github.com/libfuse/sshfs/blob/master/sshfs.rst
