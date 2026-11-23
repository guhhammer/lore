# docker

إدارة حاويات وصور Docker. بعض الأوامر الفرعية مثل `run` لها توثيق استخدام خاص بها.

## Usage
- `docker ps -a` : عرض جميع حاويات Docker (التشغيلية والمتوقفة)
- `docker run --name container_name image` : تشغيل حاوية من صورة مع اسم مخصص
- `docker container start|stop container_name` : تشغيل أو إيقاف حاوية موجودة
- `docker pull image` : سحب صورة من سجل Docker
- `docker images` : عرض قائمة الصور المحملة مسبقًا
- `docker exec -it container_name sh` : فتح وحدة تحكم تفاعلية داخل حاوية تعمل مع شيل Bourne (`sh`)
- `docker rm container1 container2 ...` : إزالة الحاويات المتوقفة
- `docker logs -f container_name` : متابعة وعرض سجلات الحاوية

## Links
- More information: https://docs.docker.com/reference/cli/docker/
