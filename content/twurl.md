# twurl

Curl-like command but tailored specifically for the Twitter API.

## Usage
- `twurl authorize -c twitter_api_key -s twitter_api_secret` : authorize `twurl` to access a Twitter account
- `twurl -X GET twitter_api_endpoint` : make a GET request to an API endpoint
- `twurl -X POST -d 'endpoint_params' twitter_api_endpoint` : make a POST request to an API endpoint
- `twurl -H "twitter_upload_url" -X POST "twitter_upload_endpoint" -f "path/to/media.jpg" -F "media"` : upload media to Twitter
- `twurl -H twitter_api_url -X GET twitter_api_endpoint` : access a different Twitter API host
- `twurl alias alias_name resource` : create an alias for a requested resource

## Categories
network

## Links
- More information: https://github.com/twitter/twurl#getting-started
- tldr page: https://tldr.inbrowser.app/pages/common/twurl
