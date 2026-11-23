# usermod

사용자 계정을 수정합니다.

## Usage
- `sudo usermod -l 새로운_사용자명 사용자명` : 사용자명을 변경
- `sudo usermod -u id 사용자명` : 사용자 ID 변경
- `sudo usermod -s 경로/대상/셸 사용자명` : 사용자 셸 변경
- `sudo usermod -a -G 그룹1,그룹2,... 사용자명` : 사용자를 보조 그룹에 추가 (공백 없음에 유의)
- `sudo usermod -m -d 경로/대상/새로운_홈 사용자명` : 사용자 홈 디렉터리 변경

## Links
- More information: https://manned.org/usermod
