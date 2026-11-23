# locust

Load-testing tool to determine number of concurrent users a system can handle.

## Usage
- `locust -H http://example.com` : load-test "example.com" with web interface using `locustfile.py`
- `locust -H http://example.com -f test_file.py` : use a different test file
- `locust -H http://example.com --headless -u 100 -r 1` : run test without web interface, spawning 1 user a second until there are 100 users
- `locust -H http://example.com --master` : start Locust in master mode
- `locust -H http://example.com --worker` : connect Locust worker to master
- `locust -H http://example.com --worker --master-host master_hostname` : connect Locust worker to master on a different machine

## Categories
network, users, monitoring

## Links
- More information: https://docs.locust.io/en/stable/configuration.html#configuration
- tldr page: https://tldr.inbrowser.app/pages/common/locust
