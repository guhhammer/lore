# fswebcam

작고 간단한 \*nix용 웹캠.

## Usage
- `fswebcam 파일이름` : 사진을 찍음
- `fswebcam -r 너비x높이 파일이름` : 사용자 정의 해상도로 사진 찍기
- `fswebcam -d 장치 파일이름` : 선택한 장치에서 사진을 찍음(기본값 `/dev/video0`)
- `fswebcam --timestamp 타임스탬프 파일이름` : 타임스탬프가 있는 사진을 찍음(타임스탬프 문자열은 strftime로 형식화됨)

## Links
- More information: https://www.sanslogic.co.uk/fswebcam/
