# komac

`winget-pkgs` 저장소용 WinGet 매니페스트 생성 .

## Usage
- `komac new Package.Identifier -v 1.2.3 -u https://example.com/app.exe` : 새로운 패키지를 처음부터 생성
- `komac update Package.Identifier -v 1.2.3 -u https://example.com/app.exe` : 기존 패키지를 새로운 버전으로 업데이트
- `komac update Package.Identifier -v 1.2.3 -u https://example.com/app.exe https://example.com/app.msi ... -s` : 여러 URL을 사용하여 패키지를 업데이트하고 자동 제출 수행
- `komac remove Package.Identifier -v 1.2.3` : winget-pkgs에서 특정 버전 제거
- `komac list Package.Identifier` : 패키지의 모든 버전 목록 표시
- `komac sync` : 자신의 winget-pkgs 포크를 상위 저장소와 동기화
- `komac token add -t 당신의_Github_토큰` : 저장된 GitHub 토큰 추가 또는 업데이트
- `komac complete bash|elvish|fish|powershell|zsh` : 쉘 자동완성 스크립트 생성

## Links
- More information: https://github.com/russellbanks/Komac
