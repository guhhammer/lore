# in-toto-sign

in-toto 링크 또는 레이아웃 메타데이터에 서명하거나 서명을 검증하는 도구.

## Usage
- `in-toto-sign -f unsigned.layout -k 개인키1 개인키2 -o root.layout` : 두 개의 키로 'unsigned.layout'에 서명하고 'root.layout'로 저장
- `in-toto-sign -f package.2f89b927.link -k 개인키` : 링크 파일의 서명을 교체하고 기본 파일명으로 저장
- `in-toto-sign -f root.layout -k 공개키0 공개키1 공개키2 --verify` : 3개의 키로 서명된 레이아웃 검증
- `in-toto-sign -f root.layout -g` : 기본 GPG 키를 사용하여 레이아웃 서명
- `in-toto-sign -f root.layout --verify -g ...439F3C2` : 특정 keyid('...439F3C2')의 GPG 키로 레이아웃 검증
