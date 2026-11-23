# mount

하나의 폴더에 있는 전체 파일 시스템에 대한 접근을 제공합니다.

## Usage
- `mount` : 모든 마운트된 파일 시스템 표시
- `mount 경로/대상/장치_파일 경로/대상/목표_디렉토리` : 장치를 디렉토리에 마운트
- `mount -m 경로/대상/장치_파일 경로/대상/목표_디렉토리` : 존재하지 않을 때 특정 폴더를 생성하고, 해당 폴더에 장치를 마운트
- `mount -o uid=사용자_id,gid=그룹_id 경로/대상/장치_파일 경로/대상/목표_디렉토리` : 특정 사용자로 장치를 디렉토리에 마운트
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : CD-ROM 장치(파일 타입 ISO9660)를 `/cdrom`에 마운트 (읽기 전용)
- `mount -a` : `/etc/fstab`에 정의된 모든 파일 시스템을 마운트
- `mount 경로/대상/마운트_포인트` : `/etc/fstab`에 설정된 특정 파일 시스템을 마운트 (예, `/dev/sda1 /my_drive ext2 defaults 0 2`)
- `mount -B 경로/대상/기존_디렉토리 경로/대상/새로운_디렉토리` : 디렉토리를 다른 디렉토리에 마운트

## Links
- More information: https://manned.org/mount.8
