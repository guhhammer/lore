# tshark

패킷 분석 도구, Wireshark의 CLI 버전.

## Usage
- `tshark` : 로컬호스트에서 모든 것 모니터링
- `tshark -f 'udp port 53'` : 특정 캡처 필터와 일치하는 패킷만 캡처
- `tshark -Y 'http.request.method == "GET"'` : 특정 출력 필터와 일치하는 패킷만 표시
- `tshark -d tcp.port==8888,http` : 특정 프로토콜(예: HTTP)로 TCP 포트 디코딩
- `tshark -T json|text|ps|...` : 캡처된 출력의 형식 지정
- `tshark -T fields|ek|json|pdml -e http.request.method -e ip.src` : 출력할 특정 필드 선택
- `tshark -w 경로/대상/파일` : 캡처된 패킷을 f파일에 저장
- `tshark -r 경로/대상/파일.pcap` : f파일에서 패킷 분석

## Links
- More information: https://tshark.dev/#sitemap-in-tshark---help
