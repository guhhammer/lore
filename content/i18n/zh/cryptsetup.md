# cryptsetup

管理普通 dm-crypt 和 LUKS（Linux 统一密钥设置）加密卷。

## Usage
- `cryptsetup luksFormat /dev/sda1` : 初始化 LUKS 卷（覆盖分区上的所有数据）
- `cryptsetup luksOpen /dev/sda1 目标` : 打开 LUKS 卷并在 `/dev/mapper/目标` 创建解密映射
- `cryptsetup luksClose 目标` : 删除已存在的映射
- `cryptsetup luksChangeKey /dev/sda1` : 更改 LUKS 卷的口令

## Links
- More information: https://manned.org/cryptsetup
