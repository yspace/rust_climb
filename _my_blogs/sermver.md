> Cargo supports caret (^x.y.z), tilde (~x.y.z), wildcard (*, x.*),     comparison requirements (>=x, <x.y, =x.y.z), and combinations thereof

用= 会锁版本 其他情况使用  `cargo update` 会更新依赖 除非有特殊原因尽量不要锁定版本号 优先使用最新库

其他语言中也有类似的库依赖机制 理念都类似

尽量不要吧Cargo.lock 包含进版本控制中 应将其加入.gitignore 其他语言也应如此