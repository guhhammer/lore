# bdfr

Bulk downloader for Reddit.

## Usage
- `bdfr download path/to/output_directory -l post_url` : download videos/images from the specified links to URL or ID's of posts
- `bdfr download path/to/output_directory -u reddit_user --submitted` : download the maximum possible number (roughly 1000) of videos/images from a specified user
- `bdfr archive path/to/output_directory -s 'Python, all, mindustry' -L 10` : download submission data (text, upvotes, comments, etc.) limited to 10 submissions for each subreddit (30 total)
- `bdfr download path/to/output_directory -s Python -S top -t all -L 10` : download videos/images from the subreddit r/Python sorted by top (default is hot) using time filter all, limited to 10 submissions
- `bdfr clone path/to/output_directory -s Python --skip mp4 --skip gif --make-hard-links` : download the maximum possible number of both submission data and videos/images from subreddit r/Python skipping over submissions with `.mp4` or `.gif` file extensions and creating hard links for duplicate files
- `bdfr download path/to/output_directory -u me --saved --authenticate --file-scheme ' {POSTID}_{TITLE}_{UPVOTES} ' --no-dupes --search-existing` : download saved posts of the authenticated user, naming each file according to a specified format. Avoid downloading duplicates and posts already present in the output directory

## Categories
network, files, containers

## Links
- More information: https://github.com/Serene-Arc/bulk-downloader-for-reddit
- tldr page: https://tldr.inbrowser.app/pages/common/bdfr
