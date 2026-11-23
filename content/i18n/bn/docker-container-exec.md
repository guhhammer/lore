# docker container exec

ইতিমধ্যে চলমান একটি Docker কন্টেইনারে কমান্ড চালান।

## Usage
- `docker exec -it container_name /bin/bash` : ইতিমধ্যে চলমান একটি কন্টেইনারে একটি ইন্টারঅ্যাক্টিভ শেল সেশনে প্রবেশ করুন
- `docker exec -d container_name কমান্ড` : চলমান কন্টেইনারে ব্যাকগ্রাউন্ডে (detached) একটি কমান্ড চালান
- `docker exec -it -w ডিরেক্টরি/এর/পাথ container_name কমান্ড` : একটি কমান্ড চালানোর জন্য নির্দিষ্ট working directory নির্বাচন করুন
- `docker exec -i -d container_name কমান্ড` : চলমান কন্টেইনারে ব্যাকগ্রাউন্ডে একটি কমান্ড চালান, কিন্তু `stdin` খোলা রাখুন
- `docker exec -it -e variable_name=ভ্যালু container_name /bin/bash` : চলমান Bash সেশনে একটি environment variable সেট করুন
- `docker exec -u user container_name কমান্ড` : নির্দিষ্ট ব্যবহারকারী হিসেবে একটি কমান্ড চালান

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
