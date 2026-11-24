# kubectl

kubectl talks to a Kubernetes cluster: see what runs, read logs, change and roll back workloads.

## Tasks
### Choose the cluster
- `kubectl config get-contexts` : list clusters you can use
- `kubectl config use-context NAME` : switch cluster
- `kubectl config set-context --current --namespace=NAME` : set the default namespace
### Look
- `kubectl get pods` : pods in the current namespace
- `kubectl get pods -A` : pods in every namespace
- `kubectl get pods -o wide` : with node and IP
- `kubectl get all -n NAME` : the main objects of a namespace
- `kubectl get deploy,svc,ing` : deployments, services and ingresses
- `kubectl describe pod NAME` : details and recent events
- `kubectl get events --sort-by=.lastTimestamp` : what happened lately
- `kubectl top pods` : CPU and memory (needs metrics-server)
### Logs and shells
- `kubectl logs -f NAME` : follow a pod's logs
- `kubectl logs NAME -c CONTAINER --previous` : logs of the last crashed container
- `kubectl exec -it NAME -- sh` : open a shell in a pod
- `kubectl port-forward pod/NAME 8080:80` : reach a pod on localhost:8080
- `kubectl cp NAME:/path/file .` : copy a file out of a pod
### Apply and change
- `kubectl apply -f FILE.yaml` : create or update from a file
- `kubectl apply -f DIR/` : apply every file in a folder
- `kubectl diff -f FILE.yaml` : show what apply would change
- `kubectl delete -f FILE.yaml` : delete what a file describes (deletes it)
- `kubectl scale deploy NAME --replicas=3` : change the number of pods
- `kubectl set image deploy/NAME CONTAINER=IMAGE:TAG` : roll out a new image
- `kubectl edit deploy NAME` : edit a live object in your editor
### Roll out and back
- `kubectl rollout status deploy/NAME` : wait for a rollout to finish
- `kubectl rollout history deploy/NAME` : list revisions
- `kubectl rollout undo deploy/NAME` : go back to the previous revision
- `kubectl rollout restart deploy/NAME` : restart every pod, one by one
### Secrets and debugging
- `kubectl create secret generic NAME --from-literal=KEY=value` : make a secret
- `kubectl get secret NAME -o jsonpath='{.data.KEY}' | base64 -d` : read one value

## Notes
- Add `-n NAME` to any command to use another namespace.
- `kubectl delete` removes real workloads at once: check the context with `kubectl config current-context` first.
- Secrets are only base64-encoded, not encrypted.
- `kubectl get -o yaml` shows an object as the cluster has it, including generated fields.

## See also
- helm
- docker
- k9s

## Links
- Cheat sheet: https://kubernetes.io/docs/reference/kubectl/quick-reference/
- Reference: https://kubernetes.io/docs/reference/kubectl/

## Categories
containers cloud
