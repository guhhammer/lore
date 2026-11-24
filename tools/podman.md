# podman

Podman runs containers like Docker, without a background daemon and usually without root. Its commands match docker's, so most habits carry over.

## Tasks
### Run
- `podman run --rm -it ubuntu bash` : start a throwaway container with a shell
- `podman run -d --name NAME -p 8080:80 nginx` : run in the background, port 8080 to 80
- `podman run -d -v VOLUME:/data:Z IMAGE` : mount a volume (`:Z` relabels it on SELinux systems)
- `podman run --userns=keep-id IMAGE` : keep your user id inside the container
### Look
- `podman ps` : running containers
- `podman ps -a` : every container
- `podman logs -f NAME` : follow the logs
- `podman inspect NAME` : every detail as JSON
- `podman stats` : live resource use
### Go inside and manage
- `podman exec -it NAME bash` : open a shell in a container
- `podman stop NAME` : stop it
- `podman start NAME` : start it again
- `podman rm NAME` : delete a stopped container
- `podman rm -f NAME` : stop and delete
### Images
- `podman images` : local images
- `podman pull docker.io/library/nginx` : download an image
- `podman build -t NAME .` : build from a Containerfile or Dockerfile
- `podman push IMAGE docker://REGISTRY/NAME` : upload an image
- `podman search nginx` : look for images in registries
- `podman image prune -a` : delete unused images
### Pods and Kubernetes
- `podman pod create --name NAME -p 8080:80` : make a pod, containers that share a network
- `podman run -d --pod NAME IMAGE` : put a container in a pod
- `podman generate kube NAME > pod.yaml` : write Kubernetes YAML from a container or pod
- `podman play kube pod.yaml` : run a Kubernetes YAML file
### Run at boot
- `podman generate systemd --new --name NAME` : print a systemd unit for a container
- `systemctl --user enable --now NAME.service` : start it with your user session
- `loginctl enable-linger USER` : keep user services running without a login
### Clean up
- `podman system prune -a` : delete stopped containers and unused images (deletes them)
- `podman machine start` : start the virtual machine on macOS and Windows

## Notes
- Rootless ports below 1024 are blocked by default: use a high port or change `net.ipv4.ip_unprivileged_port_start`.
- `alias docker=podman` works for most commands.
- Always write full image names (`docker.io/library/nginx`); short names may ask which registry.
- Newer Podman prefers Quadlet files over `generate systemd`.
- Compose files work with `podman compose` or `podman-compose`.

## See also
- docker
- docker-compose
- buildah
- systemctl

## Links
- Docs: https://docs.podman.io/en/latest/
- Commands: https://docs.podman.io/en/latest/Commands.html

## Categories
containers
