# cs complete-dep

Search for libraries without doing it directly on the web.

## Usage
- `cs complete-dep group_id` : print which artifacts are published under a specific Maven group identifier
- `cs complete-dep group_id:artifact_id` : list published library versions under a specific Maven group identifier and an artifact one
- `cs complete-dep group_id --repository ivy2local` : print which artifacts are published under a given Maven groupId searching in the ivy2local
- `cs complete-dep group_id:artifact_id --repository repository_url --credentials user:password` : list published artifacts under a Maven group identifier searching in a specific repository and credentials

## Categories
users, packages

## Links
- More information: https://get-coursier.io/docs/cli-complete
- tldr page: https://tldr.inbrowser.app/pages/common/cs-complete-dep
