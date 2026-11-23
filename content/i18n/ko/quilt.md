# quilt

일련의 패치를 관리.

## Usage
- `quilt import 경로/대상/파일이름.patch` : 기존 패치를 파일에서 가져오기
- `quilt new 파일이름.patch` : 새 패치 생성
- `quilt add 경로/대상/파일` : 현재 패치에 파일 추가
- `quilt refresh` : 파일을 편집한 후, 변경 사항으로 현재 패치 갱신
- `quilt push -a` : 시리즈 파일의 모든 패치 적용
- `quilt pop -a` : 적용된 모든 패치 제거

## Links
- More information: https://manned.org/quilt
