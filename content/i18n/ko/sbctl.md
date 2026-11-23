# sbctl

사용자 친화적인 보안 부트 키 관리자.

## Usage
- `sbctl status` : 현재 보안 부트 상태 표시
- `sbctl create-keys` : 사용자 정의 보안 부트 키 생성 (기본적으로 모든 것은 `/var/lib/sbctl`에 저장)
- `sbctl enroll-keys -m` : 사용자 정의 보안 부트 키 및 Microsoft의 UEFI 공급업체 인증서 등록
- `sbctl setup --setup` : `/etc/sbctl/sbctl.conf`의 설정에 따라 `create-keys`와 `enroll-keys` 자동 실행
- `sbctl sign -s 경로/대상/efi_바이너리` : 생성된 키로 EFI 바이너리에 서명하고 파일을 데이터베이스에 저장
- `sbctl sign-all` : 저장된 모든 파일 다시 서명
- `sbctl verify` : EFI 시스템 파티션의 모든 EFI 실행 파일이 서명되었는지 확인

## Links
- More information: https://github.com/Foxboron/sbctl/wiki/FAQ#option-rom
