# iscsiadm

iSCSI 세션, 노드 및 대상 검색을 관리.

## Usage
- `sudo iscsiadm -m session` : 활성 iSCSI 세션 목록 표시
- `sudo iscsiadm -m node` : 알려진 모든 iSCSI 노드 목록 표시
- `sudo iscsiadm -m discovery -t sendtargets -p ip_주소` : 포털에서 사용 가능한 iSCSI 대상 검색 (인증 없음)
- `sudo iscsiadm -m node -T iqn -p ip_주소:3260 -l` : 인증 없이 지정한 iSCSI 대상에 로그인
- `sudo iscsiadm -m node -T iqn -p ip_주소:3260 -u` : 지정한 iSCSI 대상에서 로그아웃
- `sudo iscsiadm -m node -o new -T iqn -p ip_주소:3260` : 대상 검색이 차단된 경우(CHAP 인증용) iSCSI 노드 생성
- `sudo iscsiadm -m node -T iqn -p ip_주소:3260 -o update -n node.session.auth.authmethod -v CHAP` : iSCSI 대상에 대해 CHAP 인증 설정

## Links
- More information: https://manned.org/iscsiadm
