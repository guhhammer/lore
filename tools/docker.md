# docker

Docker runs programs in containers: isolated, repeatable environments made from images. This page covers running, looking at, building and cleaning up.

## Tasks
### Run
- `docker run --rm -it ubuntu bash` : start a throwaway container with a shell
- `docker run -d --name NAME -p 8080:80 nginx` : run in the background, host port 8080 to container port 80
- `docker run -d -v VOLUME:/data -e KEY=value IMAGE` : mount a volume and set a variable
- `docker run -v "$PWD":/app -w /app IMAGE CMD` : run a command on the current folder
- `docker run -d --restart unless-stopped IMAGE` : start again after a reboot or crash
### Look
- `docker ps` : running containers
- `docker ps -a` : every container, stopped ones too
- `docker logs -f --tail 100 NAME` : follow the last 100 log lines
- `docker inspect NAME` : every detail as JSON
- `docker stats` : live CPU and memory use
- `docker top NAME` : processes inside a container
### Go inside
- `docker exec -it NAME bash` : open a shell in a running container
- `docker exec NAME ls /app` : run one command in it
- `docker cp NAME:/path/file .` : copy a file out of a container
- `docker cp FILE NAME:/path/` : copy a file in
### Start and stop
- `docker stop NAME` : stop it politely
- `docker start NAME` : start a stopped container
- `docker restart NAME` : stop and start
- `docker rm NAME` : delete a stopped container
- `docker rm -f NAME` : stop and delete a container
### Images
- `docker images` : local images
- `docker pull IMAGE:TAG` : download an image
- `docker build -t NAME:TAG .` : build an image from the Dockerfile here
- `docker build --no-cache -t NAME .` : build from scratch
- `docker tag NAME:TAG USER/NAME:TAG` : give it another name
- `docker push USER/NAME:TAG` : upload to a registry
- `docker rmi IMAGE` : delete an image
### Clean up
- `docker system df` : how much space Docker uses
- `docker system prune` : delete stopped containers, unused networks, dangling images
- `docker system prune -a --volumes` : delete everything unused, volumes too (deletes data)

## Notes
- Data inside a container is lost when it is removed: keep data in volumes.
- `-p HOST:CONTAINER`: the host port comes first.
- Add your user to the `docker` group to skip sudo, but that group is effectively root.
- Docker's `-p` bypasses ufw rules: published ports are open to the network.
- Install from the official page in Links; do not pipe a script into a shell.

## See also
- docker-compose
- podman
- kubectl

## Links
- CLI reference: https://docs.docker.com/reference/cli/docker/
- Install: https://docs.docker.com/engine/install/

## Categories
containers
