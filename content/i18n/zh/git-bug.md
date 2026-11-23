# git bug

一个使用 Git 内部存储的分布式问题追踪器，不会在项目中添加额外文件。 您可以像提交和分支一样，将问题提交到与他人交互的同一个 Git 远程仓库。

## Usage
- `git bug user create` : 创建新身份
- `git bug add` : 创建新问题
- `git bug push` : 推送新问题条目到远程仓库
- `git bug pull` : 拉取更新
- `git bug ls` : 列出已有问题
- `git bug ls "状态:open 排序:edit"` : 使用查询条件筛选和排序问题
- `git bug ls "搜索查询" baz` : 按文本内容搜索问题

## Links
- More information: https://github.com/git-bug/git-bug/blob/master/doc/md/git-bug.md
