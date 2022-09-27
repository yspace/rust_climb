默认安装
> cargo install diesel_cli

会报错 mysqlclient库缺失

只安装特定支持：
>  cargo install diesel_cli --no-default-features --features "sqlite"

不依赖 .env 和环境变量 手动指定 但每个命令都得加这个选项 有点烦！
> diesel  --database-url "sqlite://my.db?mode=rwc" setup


一个不错的sqlite 工具
https://sqlitebrowser.org/dl/