# ccache

C/C++ 컴파일러 캐시. 참고: 패키지는 일반적으로 `/usr/lib/ccache/bin`에 컴파일러에 대한 심볼릭 링크를 제공. 자동으로 `ccache`를 사용하려면 이 디렉토리를 `$PATH` 앞에 추가.

## Usage
- `ccache -s` : 현재 캐시 통계 표시(statistics)
- `ccache -C` : 모든 캐시 지우기(Clear)
- `ccache -z` : 통계 재설정 (zero) (캐시 자체는 아님)
- `ccache gcc 경로/대상/파일.c` : C 코드를 컴파일하고 컴파일된 출력을 캐시 (모든 `gcc` 호출에서 `ccache`를 사용하려면, 위를 참고)

## Links
- More information: https://ccache.dev/manual/latest.html
