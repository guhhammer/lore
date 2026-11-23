# wondershaper

네트워크 어댑터의 대역폭을 제한할 수 있도록 합니다.

## Usage
- `wondershaper -s -a 어댑터_이름` : 특정 adapter의 현재 status 보기
- `wondershaper -c -a 어댑터_이름` : 특정 adapter의 제한 해제
- `wondershaper -a 어댑터_이름 -d 1024` : 특정 최대 download 속도 설정 (Kbps 단위)
- `wondershaper -a 어댑터_이름 -u 512` : 특정 최대 upload 속도 설정 (Kbps 단위)
- `wondershaper -a 어댑터_이름 -d 1024 -u 512` : 특정 최대 download 및 upload 속도 설정 (Kbps 단위)
- `wondershaper -h` : help 표시

## Links
- More information: https://github.com/magnific0/wondershaper#usage
