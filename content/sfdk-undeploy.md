# sfdk undeploy

Undeploy build results from a device.

## Usage
- `sfdk undeploy --pkcon|rpm|rsync|sdk|zypper` : undeploy using a specified method
- `sfdk undeploy --method -n` : simulate an undeployment without applying the changes
- `sfdk undeploy --method "+package*"` : undeploy files in `glob` pattern `package*`
- `sfdk undeploy --method "-ignore*"` : undeploy all files excluding `ignore*`

## Categories
development, hardware

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sfdk-undeploy
