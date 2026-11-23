# koji call

Esegue una chiamata XML-RPC arbitraria. Es: la dichiarazione della funzione build è build(src, target, opts=None, priority=None, channel=None).

## Usage
- `koji call build '"git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf"' \"target\" --kwargs '{"opts":{"scratch": True\}\}'` : chiama la funzione build con opzione scratch
- `koji call build '"git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf"' \"target\" --kwargs '{"opts":{"arch_override":"sw_64"\}\}'` : chiama la funzione build con opzione arch_override
- `koji call build '"git+https://src.fedoraproject.org/rpms/vim.git#e847a50297a216229050bf4db3d06a139104e7cf"' \"target\" --kwargs '{"channel":"default"}'` : chiama la funzione build sul canale `default`
- `koji call -h` : mostra l'aiuto

## Links
- More information: https://koji.fedoraproject.org/koji/api
