# kubectl autoscale

Create an autoscaler to intelligently scale pod count based on kubernetes cluster demands.

## Usage
- `kubectl autoscale deploy deployment_name --min min_replicas --max max_replicas` : auto scale a deployment with no target CPU utilization specified
- `kubectl autoscale deploy deployment_name --max max_replicas --cpu-percent target_cpu` : auto scale a deployment with target CPU utilization

## Categories
containers, hardware

## Links
- More information: https://kubernetes.io/docs/reference/kubectl/generated/kubectl_autoscale/
- tldr page: https://tldr.inbrowser.app/pages/common/kubectl-autoscale
