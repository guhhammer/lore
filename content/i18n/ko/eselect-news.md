# eselect news

Gentoo 뉴스 항목을 읽기 위한 `eselect` 모듈. 참고: 저장소가 동기화되고 읽지 않은 뉴스 항목이 있을 때 Portage가 알림을 출력합니다.

## Usage
- `eselect news list all|new` : 사용 가능한 뉴스 항목과 번호 나열 (기본적으로 모두)
- `eselect news read 번호1 번호2 ...` : 지정된 뉴스 항목 출력
- `eselect news read` : 읽지 않은 모든 뉴스 항목 출력
- `eselect news unread 번호1 번호2 ...` : 지정된 뉴스 항목을 읽지 않음으로 표시
- `eselect news purge` : 읽은 모든 뉴스 항목 삭제
- `eselect news count all|new` : 사용 가능한 뉴스 항목 수 출력 (기본적으로 새 항목)

## Links
- More information: https://wiki.gentoo.org/wiki/Eselect#News
