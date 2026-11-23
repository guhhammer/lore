# fsutil

파일 시스템 볼륨에 대한 정보를 표시.

## Usage
- `fsutil volume list` : 볼륨 목록 표시
- `fsutil fsInfo volumeInfo 드라이브_문자|볼륨_경로` : 볼륨의 파일 시스템 정보 표시
- `fsutil repair state` : 모든 볼륨의 파일 시스템 자동 복구 현재 상태 표시
- `fsutil dirty query` : 모든 볼륨의 더티 비트 상태 표시
- `fsutil dirty set 드라이브_문자|볼륨_경로` : 볼륨의 더티 비트 상태 설정
