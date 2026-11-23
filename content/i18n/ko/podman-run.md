# podman run

새로운 Podman 컨테이너에서 명령 실행.

## Usage
- `podman run 이미지:태그 명령` : 태그된 이미지에서 새로운 컨테이너로 명령 실행
- `podman run -d 이미지:태그 명령` : 백그라운드에서 새로운 컨테이너로 명령 실행하고 ID 표시
- `podman run --rm -it 이미지:태그 명령` : 일회성 컨테이너에서 대화형 모드와 가상 터미널로 명령 실행
- `podman run -e '변수=값' -e 변수 이미지:태그 명령` : 환경 변수를 전달하여 새로운 컨테이너로 명령 실행
- `podman run -v /경로/대상/호스트_경로:/경로/대상/컨테이너_경로 이미지:태그 명령` : 바인드 마운트된 볼륨으로 새로운 컨테이너에서 명령 실행
- `podman run -p 호스트_포트:컨테이너_포트 이미지:태그 명령` : 포트가 공개된 새로운 컨테이너에서 명령 실행
- `podman run --entrypoint 명령 이미지:태그` : 이미지의 엔트리포인트를 덮어쓰며 새로운 컨테이너에서 명령 실행
- `podman run --network 네트워크 이미지:태그` : 네트워크에 연결된 새로운 컨테이너에서 명령 실행

## Links
- More information: https://docs.podman.io/en/latest/markdown/podman-run.1.html
