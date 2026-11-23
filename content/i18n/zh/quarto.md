# quarto

一个基于 Pandoc 的开源科学和技术出版系统。

## Usage
- `quarto create-project 路径/到/目录 --type book|default|website` : 创建一个新项目
- `quarto create-project 路径/到/目录 --type website --template blog` : 创建一个新的博客网站
- `quarto render 路径/到/文件.ext --to html|pdf|docx` : 将输入文件渲染为不同格式
- `quarto preview 路径/到/目录 | 路径/到/文件` : 渲染并预览文档或网站
- `quarto publish quarto-pub|gh-pages|connect|netlify` : 发布文档或项目到 Quarto Pub、Github Pages、RStudio Connect 或 Netlify

## Links
- More information: https://quarto.org/docs/reference/projects/options.html
