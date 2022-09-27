https://dev.to/anshulxyz/guide-to-getting-started-with-seaorm-an-orm-for-rust-2fen

注意 环境变量的问题
sqlx 底层依赖了dotenv库 这个库会找.env文件 或者当前环境变量
为了避免.env 必须在根目录（或许当前目录？）可以在shell中先导出这个URL
> export DATABASE_URL='sqlite://posts.sqlite?mode=rwc'

.sqlite 文件有的db 工具不能识别后缀 改为db就好了
> export DATABASE_URL='sqlite://posts.db?mode=rwc'

运行

~~~shell

cargo run -p db_orm --bin create_post
~~~