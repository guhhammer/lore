# docker inspect

Docker 객체에 대한 저수준 정보를 반환.

## Usage
- `docker inspect 컨테이너|이미지|id` : 이름 또는 ID를 사용하여 컨테이너, 이미지 또는 볼륨에 대한 정보 표시
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' 컨테이너` : 컨테이너의 IP 주소 표시
- `docker inspect -f '\{\{.LogPath\}\}' 컨테이너` : 컨테이너의 로그 파일 경로 표시
- `docker inspect -f '\{\{.Config.Image\}\}' 컨테이너` : 컨테이너의 이미지 이름 표시
- `docker inspect -f '\{\{json .Config\}\}' 컨테이너` : JSON 형식으로 구성 정보 표시
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' 컨테이너` : 모든 포트 바인딩 표시
- `docker inspect` : 도움말 표시

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
