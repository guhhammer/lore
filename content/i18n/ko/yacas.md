# yacas

또 다른 컴퓨터 대수 시스템.

## Usage
- `yacas` : 대화형 `yacas` 세션 시작
- `Integrate(x)Cos(x);` : [대화형] `yacas` 세션에서 문 실행
- `Example();` : [대화형] `yacas` 세션에서 예시 표시
- `quit` : [대화형] `yacas` 세션 종료
- `yacas -p -c 경로/대상/스크립트1 경로/대상/스크립트2 ...` : 하나 이상의 `yacas` 스크립트를 실행하고(터미널이나 프롬프트 없이) 종료
- `echo "Echo( Deriv(x)Cos(1/x) );" | yacas -p -c /dev/stdin` : 하나의 문을 실행하고 결과를 출력한 후 종료

## Links
- More information: https://www.yacas.org/
