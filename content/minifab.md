# minifab

Automate the setup and deployment of Hyperledger Fabric networks.

## Usage
- `minifab up -i minifab_version` : bring up the default Hyperledger Fabric network
- `minifab down` : bring down the Hyperledger Fabric network
- `minifab install -n chaincode_name` : install chaincode onto a specified channel
- `minifab install -n chaincode_name -v chaincode_version` : install a specific chaincode version onto a channel
- `minifab approve,commit,initialize,discover` : initialize the chaincode after installation/upgrade
- `minifab invoke -n chaincode_name -p '"method_name", "argument1", "argument2", ...'` : invoke a chaincode method with the specified arguments
- `minifab blockquery block_number` : make a query on the ledger
- `minifab apprun -l app_programming_language` : quickly run an application

## Categories
packages, network

## Links
- More information: https://github.com/hyperledger-labs/minifabric
- tldr page: https://tldr.inbrowser.app/pages/common/minifab
