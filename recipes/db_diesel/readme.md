默认安装
> cargo install diesel_cli

会报错 mysqlclient库缺失

只安装特定支持：
>  cargo install diesel_cli --no-default-features --features "sqlite"

不依赖 .env 和环境变量 手动指定 但每个命令都得加这个选项 有点烦！
> diesel  --database-url "sqlite://my.db?mode=rwc" setup


运行migration
> diesel --database-url "sqlite://my.db"  migration run
重新回滚并运行 重做
> diesel --database-url "sqlite://my.db"  migration redo

一个不错的sqlite 工具
https://sqlitebrowser.org/dl/


## 运行例子
>  export DATABASE_URL='sqlite://my.db?mode=rwc' && cargo run -p db_diesel --bin show_posts

发布
> cargo run -p db_diesel --bin publish_post 1