# mvn deploy

Add an artifact to a remote repository.

## Usage
- `mvn deploy` : copy the final artifact into the remote repository configured in the `settings.xml` file
- `mvn deploy:deploy-file -D url=URLOfTheRemoteRepository -D repositoryId=ServerIdFromSettingsXML -D file=FileToBeDeployed` : copy an artifact, that is not built using Maven to the remote repository

## Categories
files, packages, network

## Links
- More information: https://manned.org/mvn
- tldr page: https://tldr.inbrowser.app/pages/common/mvn-deploy
