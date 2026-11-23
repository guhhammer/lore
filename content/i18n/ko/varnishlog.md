# varnishlog

Varnish 로그 표시.

## Usage
- `varnishlog` : 실시간으로 로그 표시
- `varnishlog -q 'ReqHeader eq "Host: example.com"'` : 특정 도메인에 대한 요청만 표시
- `varnishlog -q 'ReqMethod eq "POST"'` : POST 요청만 표시
- `varnishlog -q 'ReqURL eq "/경로"'` : 특정 경로에 대한 요청만 표시
- `varnishlog -q 'ReqURL ~ "정규표현식"'` : 정규 표현식과 일치하는 경로에 대한 요청만 표시

## Links
- More information: https://vinyl-cache.org/docs/trunk/reference/varnishlog.html
