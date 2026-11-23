# cosign

Container Signing, Verification, and Storage in an OCI registry.

## Usage
- `cosign generate-key-pair` : generate a key-pair
- `cosign sign --key cosign.key image` : sign a container and store the signature in the registry
- `cosign sign --key k8s://namespace/key image` : sign a container image with a key pair stored in a Kubernetes secret
- `cosign sign-blob --key cosign.key path/to/file` : sign a blob with a local key pair file
- `cosign verify --key cosign.pub image` : verify a container against a public key
- `cosign dockerfile verify -key cosign.pub path/to/Dockerfile` : verify images with a public key in a Dockerfile
- `cosign verify --key k8s://namespace/key image` : verify an image with a public key stored in a Kubernetes secret
- `cosign copy example.com/src:latest example.com/dest:latest` : copy a container image and its signatures

## Categories
containers, media, files

## Links
- More information: https://github.com/sigstore/cosign/blob/main/doc/cosign.md
- tldr page: https://tldr.inbrowser.app/pages/common/cosign
