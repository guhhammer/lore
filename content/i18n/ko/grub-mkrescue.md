# grub-mkrescue

GRUB 용 부팅 가능한 CD/USB/floppy 이미지를 생성.

## Usage
- `grub-mkrescue --output grub.iso .` : 현재 디렉터리에서 부팅 가능한 ISO를 생성해 `grub.iso`로 저장
- `grub-mkrescue --directory /usr/lib/grub/i386-pc --output grub.iso 경로/대상/소스파일` : 사용자 지정 디렉터리의 GRUB 파일을 사용하여 ISO 생성
- `grub-mkrescue --compress no|xz|gz|lzo --output grub.iso 경로/대상/소스파일` : 이미지 생성 시 GRUB 파일 압축 방식 지정, (`no`: 압축 비활성화)
- `grub-mkrescue --disable-cli --output grub.iso 경로/대상/소스파일` : 생성된 이미지에서 GRUB 명령줄 인터페이스 비활성화
- `grub-mkrescue --modules "part_gpt iso9660" --output grub.iso 경로/대상/소스파일` : 지정한 GRUB 모듈을 이미지에 미리 포함
- `grub-mkrescue --output grub.iso -- -volid 볼륨_이름 경로/대상/소스파일` : 추가 옵션을 `xorriso`에 직접 전달
- `grub-mkrescue -?` : 도움말 표시
- `grub-mkrescue --version` : 버전 표시
