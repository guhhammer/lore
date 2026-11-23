# vboxmanage-extpack

Manage extension packs for Oracle VirtualBox.

## Usage
- `VBoxManage extpack install path/to/file.vbox-extpack` : install extension packs to VirtualBox (Note: You need to remove the existing version of the extension pack before installing a new version.)
- `VBoxManage extpack install --replace` : remove the existing version of the VirtualBox extension pack
- `VBoxManage extpack uninstall extension_pack_name` : uninstall extension packs from VirtualBox
- `VBoxManage extpack uninstall --force extension_pack_name` : uninstall extension packs and skip most uninstallation refusals
- `VBoxManage extpack cleanup` : clean up temporary files and directories left by extension packs

## Categories
files, packages

## Links
- More information: https://www.virtualbox.org/manual/ch08.html#vboxmanage-extpack
- tldr page: https://tldr.inbrowser.app/pages/common/vboxmanage-extpack
