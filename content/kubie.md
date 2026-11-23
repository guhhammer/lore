# kubie

Utility to switch between `kubectl` contexts and namespaces.

## Usage
- `kubie ctx` : display a selectable menu of contexts
- `kubie ctx context` : switch current shell to the given context
- `kubie ns namespace` : switch current shell to the given namespace
- `kubie ctx context -n namespace` : switch current shell to the given context and namespace
- `kubie exec context namespace command` : execute a command in the given context and namespace, without spawning a shell
- `kubie lint` : check the Kubernetes configuration files for issues

## Categories
shell

## Links
- More information: https://github.com/sbstp/kubie#usage
- tldr page: https://tldr.inbrowser.app/pages/common/kubie
