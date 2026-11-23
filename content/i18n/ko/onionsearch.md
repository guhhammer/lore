# onionsearch

다양한 `.onion` 검색 엔진에서 URL을 스크랩.

## Usage
- `onionsearch "문자열"` : 모든 검색 엔진에서 결과 요청
- `onionsearch "문자열" --engines tor66 deeplink phobos ...` : 특정 검색 엔진에서 검색 결과 요청
- `onionsearch "문자열" --exclude candle ahmia ...` : 검색 시 특정 검색 엔진 제외
- `onionsearch "stuxnet" --engines tor66 deeplink phobos ... --limit 3` : 엔진당 로드할 페이지 수 제한
- `onionsearch --help | grep -A 1 -i "supported engines"` : 지원되는 모든 검색 엔진 나열

## Links
- More information: https://github.com/megadose/OnionSearch#--usage
