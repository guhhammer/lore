# ytmdl

从 YouTube 下载歌曲并自动添加元数据。 从 iTunes、Spotify 等来源获取歌曲信息（艺术家、专辑、封面等）。

## Usage
- `ytmdl 歌曲名称` : 通过歌曲名称下载（交互式选择）
- `ytmdl -q 歌曲名称` : 不提示直接下载第一个结果
- `ytmdl -o 路径/到/目录 歌曲名称` : 将歌曲下载到指定目录
- `ytmdl --url https://www.youtube.com/watch?v=oHg5SJYRHA0` : 从 YouTube URL 下载歌曲
- `ytmdl --format mp3|m4a|opus 歌曲名称` : 以指定格式（mp3、m4a 或 opus）下载歌曲
- `ytmdl --artist 艺术家名称 --album 专辑名称 歌曲名称` : 下载歌曲并指定艺术家和专辑信息
- `ytmdl --list 路径/到/歌曲列表.txt` : 从文本文件中批量下载歌曲列表
- `ytmdl -h` : 显示帮助

## Links
- More information: https://github.com/deepjyoti30/ytmdl#usage
