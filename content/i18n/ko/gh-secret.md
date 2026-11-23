# gh secret

GitHub 시크릿 관리.

## Usage
- `gh secret ls` : 현재 저장소의 시크릿 키 나열
- `gh secret ls -o 조직` : 특정 조직의 시크릿 키 나열
- `gh secret ls -R 소유자/저장소` : 특정 저장소의 시크릿 키 나열
- `gh secret set 이름` : 현재 저장소에 시크릿 설정 (사용자가 값을 입력해야 함)
- `gh <경로/대상/파일 secret set 이름` : 파일에서 값을 가져와 현재 저장소에 시크릿 설정
- `gh secret set 이름 -o 조직 -r 저장소1,저장소2` : 특정 저장소에 대한 조직 시크릿 설정
- `gh secret remove 이름` : 현재 저장소의 시크릿 제거
- `gh secret remove 이름 -o 조직` : 특정 조직의 시크릿 제거

## Links
- More information: https://cli.github.com/manual/gh_secret
