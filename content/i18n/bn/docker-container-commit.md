# docker container commit

একটি কন্টেইনারের পরিবর্তন থেকে নতুন ইমেজ তৈরি করুন।

## Usage
- `docker commit container image:tag` : নির্দিষ্ট একটি কন্টেইনার থেকে একটি ইমেজ তৈরি করুন
- `docker commit -c "CMD কমান্ড" container image:tag` : তৈরি করা ইমেজে একটি `CMD` Dockerfile নির্দেশনা প্রয়োগ করুন
- `docker commit -c "ENV name=ভ্যালু" container image:tag` : তৈরি করা ইমেজে একটি `ENV` Dockerfile নির্দেশনা প্রয়োগ করুন
- `docker commit -a "author" container image:tag` : মেটাডাটায় নির্দিষ্ট লেখক (author) সহ একটি ইমেজ তৈরি করুন
- `docker commit -m "comment" container image:tag` : মেটাডাটায় নির্দিষ্ট মন্তব্য (comment) সহ একটি ইমেজ তৈরি করুন
- `docker commit -p false container image:tag` : কমিট করার সময় কন্টেইনার pause না করে একটি ইমেজ তৈরি করুন
- `docker commit --help` : সাহায্য প্রদর্শন

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
