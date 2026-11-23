# mirrord

Run code locally as if it were a pod in a Kubernetes cluster. Skip the build-push-deploy cycle when shipping features or fixing bugs against real services.

## Usage
- `mirrord exec -t pod/pod_name -- command argument1 argument2 ...` : run a local binary connected to a remote pod
- `mirrord exec -t deployment/deployment_name -n namespace -- command` : target a deployment in a specific namespace
- `mirrord exec -f path/to/mirrord.json -- command` : run with a configuration file
- `mirrord port-forward -t pod/pod_name --port-mapping local_port:remote_port` : forward a local port to a host reachable from inside the cluster
- `mirrord dump -t pod/pod_name -p 8080` : print incoming TCP traffic from specific ports on a remote target
- `mirrord wizard` : launch the interactive configuration wizard
- `mirrord diagnose` : diagnose mirrord setup and cluster connectivity

## Categories
containers, network, development

## Links
- More information: https://metalbear.com/mirrord/docs/getting-started/quick-start#cli
- tldr page: https://tldr.inbrowser.app/pages/common/mirrord
