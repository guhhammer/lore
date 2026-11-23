# gron

`JSON` 데이터를 개별 할당문 형태로 변환해 관리하기 쉽게 만드는 도구.

## Usage
- `gron 경로/대상/파일|주소` : `JSON` 파일을 개별 할당문 형태로 처리
- `gron --no-sort 경로/대상/파일|주소` : 출력 데이터를 정렬하지 않음
- `gron -k url` : 인증서 검증 비활성화
- `gron -v 경로/대상/파일|주소` : `gron` 할당문의 값을 표시
- `gron -u 경로/대상/파일|주소` : `gron`으로 변환된 할당문을 다시 `JSON`으로 복원
- `gron -s 경로/대상/파일|주소` : 입력의 각 줄을 개별 `JSON` 객체로 처리
- `gron -j 경로/대상/파일|주소` : 처리된 데이터를 `JSON` 스트림 형태로 표현

## Links
- More information: https://manned.org/gron
