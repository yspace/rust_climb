https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md

FIXME : env 文件可能只能用于项目根？
运行迁移时可以指定数据库连接串信息

> diesel migration run --database-url=postgres://yiqing:yiqing@localhost/kanban

### cargo check 报错

```shell

 --> /Users/qing/.cargo/registry/src/github.com-1ecc6299db9ec823/diesel_derives-2.0.1/src/attrs.rs:143:18
    |
143 | impl Spanned for FieldAttr {
    |                  ^^^^^^^^^ the trait `ToTokens` is not implemented for `FieldAttr`
```

https://github.com/diesel-rs/diesel/issues/3541

用这里的评论方案解决：

> n cargo update -p diesel_derives
