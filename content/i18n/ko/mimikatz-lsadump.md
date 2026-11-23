# mimikatz lsadump

Windows 로컬 보안 기관(LSA)에서 비밀 정보를 추출. SYSTEM 권한이 필요.

## Usage
- `mimikatz "lsadump::sam"` : SAM 해시 덤프
- `mimikatz "lsadump::secrets"` : SECURITY 레지스트리 하이브에서 비밀 정보 추출
- `mimikatz "lsadump::cache"` : 캐시된 도메인 자격 증명 덤프

## Links
- More information: https://github.com/gentilkiwi/mimikatz
