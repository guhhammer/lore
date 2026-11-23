# uwfmgr

통합 쓰기 필터 (UWF). 드라이브를 보호하기 위해 드라이브에 대한 모든 쓰기를 가상 오버레이로 리디렉션합니다. 기본적으로 재부팅 후 쓰기는 커밋되지 않으면 삭제됩니다.

## Usage
- `uwfmgr get-config` : 현재 상태 가져오기
- `uwfmgr volume protect 드라이브_문자:` : 드라이브를 보호로 설정
- `uwfmgr volume unprotect 드라이브_문자:` : 보호 목록에서 드라이브 제거
- `uwfmgr filter enable|disable` : 보호 사용 또는 사용 안 함 (재부팅 후 적용)
- `uwfmgr file commit 드라이브_문자:\경로\대상\파일` : 보호된 드라이브의 파일 변경 사항 커밋
- `uwfmgr file commit-delete 드라이브_문자:\경로\대상\파일` : 보호된 드라이브의 파일 삭제 커밋
