# apptainer config

Apptainer의 다양한 설정을 관리하는 명령어.

## Usage
- `sudo apptainer config fakeroot -a 사용자명` : fakeroot 사용자 매핑 추가
- `sudo apptainer config fakeroot -r 사용자명` : fakeroot 사용자 매핑 제거
- `sudo apptainer config fakeroot -e 사용자명` : fakeroot 사용자 매핑 활성화
- `sudo apptainer config fakeroot -d 사용자명` : fakeroot 사용자 매핑 비활성화
- `sudo apptainer config global -g 항목` : 전역 설정 항목의 현재 값 조회
- `sudo apptainer config global -s 항목 값` : 전역 설정 항목 값 설정
- `sudo apptainer config global -u 항목 값` : 전역 설정 항목 값 해제
- `sudo apptainer config global -r 항목` : 전역 설정 항목을 기본값으로 초기화

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_config.html
