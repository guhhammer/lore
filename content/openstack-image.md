# openstack image

OpenStack Image service, aka OpenStack Glance, allows users to upload and discover data assets meant to be used with other services.

## Usage
- `openstack image list --private|--shared|--all` : list available images
- `openstack image show --human-readable image_id` : display image details
- `openstack image create --file path/to/file --private|--shared|--all image_name` : create/upload an image
- `openstack image delete image_id1 image_id2 ...` : delete image(s)
- `openstack image save --file filename image_id` : save an image locally

## Categories
containers, media, system

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/openstack-image
