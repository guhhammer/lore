# attr

파일 시스템 객체의 확장 속성을 관리.

## Usage
- `attr -l 경로/대상/파일` : 파일의 모든 확장 속성 이름과 크기 표시
- `attr -g 속성_이름 경로/대상/파일` : 확장 속성 값 조회
- `attr -s 속성_이름 -V "속성_값" 경로/대상/파일` : 확장 속성 값 설정
- `echo "속성_값" | attr -s 속성_이름 경로/대상/파일` : `stdin`의 값을 사용하여 확장 속성 설정
- `attr -r 속성_이름 경로/대상/파일` : 확장 속성 제거

## Links
- More information: https://manned.org/attr
