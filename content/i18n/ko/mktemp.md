# mktemp

임시 파일이나 디렉토리를 생성.

## Usage
- `mktemp` : 빈 임시 파일을 생성하고 절대 경로 출력
- `mktemp -p /경로/대상/임시디렉토리` : `$TMPDIR`이 설정되지 않은 경우 사용자 지정 디렉토리 사용 (기본값은 플랫폼에 따라 다르지만 보통 `/tmp`)
- `mktemp /tmp/example.XXXXXXXX` : 사용자 지정 경로 템플릿 사용 (`X`는 무작위 영숫자로 대체됨)
- `mktemp -t example.XXXXXXXX` : 사용자 지정 파일 이름 템플릿 사용
- `mktemp -d` : 빈 임시 디렉토리를 생성하고 절대 경로 출력

## Links
- More information: https://man.openbsd.org/mktemp.1
