# urpmi

Mageia에서 패키지를 설치합니다.

## Usage
- `sudo urpmi 패키지|경로/대상/파일.rpm` : 저장소 또는 로컬 RPM 파일에서 패키지 설치
- `urpmi --no-install 패키지` : 패키지를 다운로드만 하고 설치하지 않음
- `sudo urpmi --auto-select` : 설치된 모든 패키지 업데이트 (`urpmi.update -a`를 실행하여 사용 가능한 업데이트 확인)
- `sudo urpmi --parallel local 패키지` : 네트워크의 하나 이상의 머신에서 `/etc/urpmi/parallel.cfg`에 따라 패키지 업데이트
- `sudo urpmi $(urpmq --auto-orphans -f)` : 모든 고아 패키지를 수동으로 설치됨으로 표시

## Links
- More information: https://man.linuxreviews.org/man8/urpmi.8.html
