# az redis

Manage Redis caches. Part of `azure-cli` (also known as `az`).

## Usage
- `az redis create --location location -n name -g resource_group --sku Basic|Premium|Standard --vm-size c0|c1|c2|c3|c4|c5|c6|p1|p2|p3|p4|p5` : create a new Redis cache instance
- `az redis update -n name -g resource_group --sku Basic|Premium|Standard --vm-size c0|c1|c2|c3|c4|c5|c6|p1|p2|p3|p4|p5` : update a Redis cache
- `az redis export --container container --file-format file_format -n name --prefix prefix -g resource_group` : export data stored in a Redis cache
- `az redis delete -n name -g resource_group -y` : delete a Redis cache

## Categories
databases, cloud

## Links
- More information: https://learn.microsoft.com/cli/azure/redis
- tldr page: https://tldr.inbrowser.app/pages/common/az-redis
