# docker container cp

হোস্ট এবং কন্টেইনার ফাইলসিস্টেমের মধ্যে ফাইল বা ডিরেক্টরি কপি করুন।

## Usage
- `docker cp path/to/file_or_directory_on_host container_name:path/to/file_or_directory_in_container` : হোস্ট থেকে কন্টেইনারে একটি ফাইল বা ডিরেক্টরি কপি করুন
- `docker cp container_name:path/to/file_or_directory_in_container path/to/file_or_directory_on_host` : কন্টেইনার থেকে হোস্টে একটি ফাইল বা ডিরেক্টরি কপি করুন
- `docker cp -L path/to/symlink_on_host container_name:path/to/file_or_directory_in_container` : হোস্ট থেকে কন্টেইনারে একটি ফাইল বা ডিরেক্টরি কপি করুন এবং symlink অনুসরণ করুন (symlink নিজে কপি না করে symlink করা ফাইলগুলো সরাসরি কপি করে)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/cp/
