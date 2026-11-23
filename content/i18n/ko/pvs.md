# pvs

물리 볼륨에 대한 정보를 표시합니다.

## Usage
- `sudo pvs` : 물리 볼륨에 대한 정보 표시
- `sudo pvs -a` : 비물리적 볼륨 표시
- `sudo pvs -v` : 기본 표시를 자세히 보여주도록 변경
- `sudo pvs -o 필드_이름_1,필드_이름_2,...` : 특정 필드만 표시
- `sudo pvs -o +필드_이름` : 기본 표시에 필드를 추가
- `sudo pvs --noheadings` : 헤딩 행 생략
- `sudo pvs --separator 특수_문자` : 필드 구분자 사용

## Links
- More information: https://manned.org/pvs
