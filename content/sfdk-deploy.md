# sfdk deploy

Deploy build results to a device.

## Usage
- `sfdk deploy --pkcon|rsync|sdk|zypper|zypper-dup|manual` : deploy using a specified method
- `sfdk deploy --method -n` : simulate a deployment without applying the changes
- `sfdk deploy --method "+package*"` : deploy files in `glob` pattern `package*`
- `sfdk deploy --method "-ignore*"` : deploy all files excluding `ignore*`
- `sfdk undeploy --pkcon|rpm|rsync|sdk|zypper` : undeploy using a specified method

## Categories
development, hardware

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sfdk-deploy
