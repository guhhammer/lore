# gh api

Make authenticated HTTP requests to the GitHub API and print the response.

## Usage
- `gh api repos/:owner/:repo/releases` : display the releases for the current repository in JSON format
- `gh api -H Accept:application/vnd.github.squirrel-girl-preview+json -f 'content=+1' repos/:owner/:repo/issues/123/reactions` : create a reaction for a specific issue
- `gh api graphql -F name=':repo' -f 'query'` : display the result of a GraphQL query in JSON format
- `gh api -X POST endpoint` : send a request using a custom HTTP method
- `gh api -i endpoint` : include the HTTP response headers in the output
- `gh api --silent endpoint` : do not print the response body
- `gh api --hostname github.example.com endpoint` : send a request to a specific GitHub Enterprise Server
- `gh api --help` : display the subcommand help

## Categories
network, documents, desktop

## Links
- More information: https://cli.github.com/manual/gh_api
- tldr page: https://tldr.inbrowser.app/pages/common/gh-api
