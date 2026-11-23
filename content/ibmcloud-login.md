# ibmcloud login

Log in to the IBM Cloud.

## Usage
- `ibmcloud login` : log in by using an interactive prompt
- `ibmcloud login -a api_endpoint` : log in to a specific API endpoint (default is `cloud.ibm.com`)
- `ibmcloud login -u username -p password -r us-south` : log in by providing username, password, and the targeted region as parameters
- `ibmcloud login --apikey api_key_string` : log in with an API key, passing it as an argument
- `ibmcloud login --apikey @path/to/api_key_file` : log in with an API key, passing it as a file
- `ibmcloud login --sso` : log in with a federated ID (single sign-on)

## Categories
users, monitoring, cloud

## Links
- More information: https://cloud.ibm.com/docs/cli?topic=cli-ibmcloud_cli#ibmcloud_login
- tldr page: https://tldr.inbrowser.app/pages/common/ibmcloud-login
