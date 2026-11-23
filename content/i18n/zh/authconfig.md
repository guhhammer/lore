# authconfig

用于设置系统认证资源的命令行界面。

## Usage
- `authconfig --test` : 显示当前的配置（或空运行）
- `authconfig --update --passalgo=算法名` : 设置服务器使用另一种不同的密码散列算法
- `authconfig --update --enableldapauth` : 启用 LDAP 认证
- `authconfig --update --disableldapauth` : 关闭 LDAP 认证
- `authconfig --update --enablenis` : 开启网络信息服务（NIS）
- `authconfig --update --enablekrb5` : 开启 Kerberos
- `authconfig --update --enablewinbindauth` : 开启 Winbind（活动目录）认证
- `authconfig --update --enablelocauthorize` : 开启本地认证

## Links
- More information: https://manned.org/authconfig
