# git am

패치 파일을 적용하고 커밋 생성. 이메일을 통해 커밋을 받을 때 유용합니다.

## Usage
- `git am 경로/대상/파일.patch` : 로컬 패치 파일을 적용하고 커밋
- `curl -L https://example.com/file.patch | git am` : 원격 패치 파일을 적용하고 커밋
- `git am --abort` : 패치 파일 적용 과정 중단
- `git am --reject 경로/대상/파일.patch` : 가능한 한 많은 패치 파일을 적용하고, 실패한 부분을 거부 파일로 저장

## Links
- More information: https://git-scm.com/docs/git-am
