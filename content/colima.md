# colima

Container runtimes for macOS and Linux with minimal setup.

## Usage
- `colima start` : start the daemon in the background
- `colima start --edit` : create a configuration file and use it
- `colima start --runtime containerd` : start and set up containerd (install `nerdctl` to use containerd via `nerdctl`)
- `colima start --kubernetes` : start with Kubernetes (`kubectl` is required)
- `colima start --cpu number --memory memory --disk storage_space` : customize CPU count, RAM memory, and disk space (in GiB)
- `colima list` : list containers with their information and status
- `colima status` : show runtime status

## Categories
containers, monitoring

## Links
- More information: https://github.com/abiosoft/colima
- tldr page: https://tldr.inbrowser.app/pages/common/colima
