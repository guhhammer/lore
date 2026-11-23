# smalltalkci

Framework for testing Smalltalk projects with GitHub Actions, Travis CI, AppVeyor, GitLab CI, and others.

## Usage
- `smalltalkci path/to/.smalltalk.ston` : run tests for a configuration file
- `smalltalkci` : run tests for the `.smalltalk.ston` configuration in the current directory
- `smalltalkci --headful` : debug tests in headful mode (show VM window)
- `smalltalkci --smalltalk Squeak64-Trunk` : download and prepare a well-known smalltalk image for the tests
- `smalltalkci --image path/to/Smalltalk.image --vm path/to/vm` : specify a custom Smalltalk image and VM
- `smalltalkci --clean` : clean up caches and delete builds

## Categories
development, files, containers

## Links
- More information: https://github.com/hpi-swa/smalltalkCI
- tldr page: https://tldr.inbrowser.app/pages/common/smalltalkci
