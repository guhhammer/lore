# pio account

명령줄에서 PlatformIO 계정을 관리.

## Usage
- `pio account register -u 사용자이름 -e 이메일 -p 비밀번호 --firstname 이름 --lastname 성` : 새 PlatformIO 계정 등록
- `pio account destroy` : platformIO 계정 및 관련 데이터 영구 삭제
- `pio account login -u 사용자이름 -p 비밀번호` : platformIO 계정에 로그인
- `pio account logout` : platformIO 계정에서 로그아웃
- `pio account update -u 사용자이름 -e 이메일 --firstname 이름 --lastname 성 --current-password 비밀번호` : platformIO 프로필 업데이트
- `pio account show` : platformIO 계정에 대한 자세한 정보 표시
- `pio account forgot -u 사용자이름_또는_이메일` : 사용자 이름이나 이메일을 사용하여 비밀번호 재설정

## Links
- More information: https://docs.platformio.org/en/latest/core/userguide/account/
