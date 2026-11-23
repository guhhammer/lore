# koji call

Execute an arbitrary XML-RPC call. Example: a build function declaration is `build(src, target, opts=None, priority=None, channel=None)`.

## Usage
- `koji call build '"git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf"' \"target\" --kwargs '{"opts":{"scratch": True\}\}'` : call build function with scratch option
- `koji call build '"git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf"' \"target\" --kwargs '{"opts":{"arch_override":"sw_64"\}\}'` : call build function with arch_override option
- `koji call build '"git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf"' \"target\" --kwargs '{"channel":"default"}'` : call build function on `default` channel
- `koji call -h` : display help

## Categories
development, version-control, network

## Links
- More information: https://koji.fedoraproject.org/koji/api
- tldr page: https://tldr.inbrowser.app/pages/linux/koji-call
