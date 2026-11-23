# carbon-now

아름다운 코드 이미지 생성.

## Usage
- `carbon-now 경로/대상/파일` : 기본 설정을 사용하여 파일에서 이미지를 만듬
- `carbon-now --from-clipboard` : 기본 설정을 사용해 클립보드의 텍스트에서 이미지를 생성
- `입력 | carbon-now --to-clipboard` : 기본 설정을 사용해 `stdin`에서 이미지를 만들고 클립보드에서 복사
- `carbon-now -i 경로/대상/파일` : 사용자 정의 설정을 위해 대화형으로(interactively) 이미지를 생성하고, 선택적으로 사전 설정을 저장
- `carbon-now -p preset 경로/대상/파일` : 이전에 저장된 프리셋(preset)에서 이미지 생성
- `carbon-now -s line 경로/대상/파일` : 지정된 텍스트 줄에서 시작(start)
- `carbon-now -e line 경로/대상/파일` : 지정된 텍스트 줄에서 종료(end)
- `carbon-now --open 경로/대상/파일` : 이미지를 저장하는 대신, 브라우저에서 열기

## Links
- More information: https://github.com/mixn/carbon-now-cli
