# ffmpeg

Video dönüştürme aracı.

## Usage
- `ffmpeg -i yol/video.mp4 -vn yol/ses.mp3` : bir videodaki sesi çıkarıp MP3 olarak kaydet
- `ffmpeg -i yol/girdi_ses.flac -ar 44100 -sample_fmt s16 yol/çıktı_ses.wav` : bir FLAC dosyasını Red Book CD biçimine dönüştür (44100 Hz, 16 bit)
- `ffmpeg -i yol/video.mp4 -vf 'scale=-1:1000' -r 15 yol/çıktı.gif` : bir videoyu, yüksekliği 1000 px ve kare hızı 15 olacak şekilde GIF olarak kaydet
- `ffmpeg -f image2 -i yol/kare_%d.jpg video.mpg|video.gif` : numaralı görüntüleri (`kare_1.jpg`, `kare_2.jpg` vb.) bir video ya da GIF olarak birleştir
- `ffmpeg -i yol/girdi_video.mp4 -ss mm:ss -to mm2:ss2 -c copy yol/çıktı_video.mp4` : bir videoyu mm:ss başlangıç zamanından mm2:ss2 bitiş zamanına kadar kırp (sonuna kadar kırpmak için `-to` seçeneğini kaldır)
- `ffmpeg -i yol/girdi_video.avi -c:a aac -b:a 128k -c:v libx264 -crf 23 yol/çıktı_video.mp4` : AVI videoyu MP4'e dönüştür (AAC ses @ 128 kbit, h264 video @ CRF 23)
- `ffmpeg -i yol/girdi_video.mkv -c copy yol/çıktı_video.mp4` : MKV videoyu ses ve video akışlarını yeniden kodlamadan MP4'e aktar
- `ffmpeg -i yol/girdi_video.mp4 -c:v libvpx-vp9 -crf 30 -b:v 0 -c:a libopus -vbr on -threads iş_parçacığı_sayısı yol/çıktı_video.webm` : MP4 videoyu VP9 codec'ine dönüştür. En iyi kalite için bir CRF değeri kullan (önerilen aralık 15-35); `-b:v` mutlaka 0 olmalıdır

## Links
- More information: https://ffmpeg.org/ffmpeg.html#Options
