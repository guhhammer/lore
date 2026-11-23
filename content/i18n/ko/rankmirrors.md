# rankmirrors

Pacman 미러 목록을 연결 및 열기 속도에 따라 순위 매기기. 새로운 미러리스트를 `stdout`에 작성합니다.

## Usage
- `rankmirrors /etc/pacman.d/mirrorlist` : 미러 목록 순위 매기기
- `rankmirrors -n 개수 /etc/pacman.d/mirrorlist` : 상위 순위 서버의 지정된 개수만 출력
- `rankmirrors -v /etc/pacman.d/mirrorlist` : 미러리스트 생성 시 자세히 출력
- `rankmirrors -u url` : 특정 URL만 테스트
- `rankmirrors -t /etc/pacman.d/mirrorlist` : 전체 미러리스트 대신 응답 시간만 출력

## Links
- More information: https://manned.org/rankmirrors
