# instaloader

Download pictures, videos, captions, and other metadata from Instagram. Note: You will need to provide Instagram login information if you want high-quality media downloads.

## Usage
- `instaloader profile_name` : download a profile
- `instaloader --highlights profile_name` : download highlights
- `instaloader -q -G profile_name` : download posts with geotags (if available), suppressing any user interaction
- `instaloader --user-agent user_agent profile_name` : specify a user agent for HTTP requests
- `instaloader -l username -p password profile_name` : specify login info and download posts (useful for private profiles)
- `instaloader -F profile_name` : skip a target if the first downloaded file has been found (useful for updating Instagram archives)
- `instaloader -l username -p password -s --igtv profile_name` : download stories and IGTV videos (login required)
- `instaloader -l username -p password -s --igtv --highlights profile_name` : download all types of posts (login required)

## Categories
network, users

## Links
- More information: https://instaloader.github.io/cli-options.html
- tldr page: https://tldr.inbrowser.app/pages/linux/instaloader
