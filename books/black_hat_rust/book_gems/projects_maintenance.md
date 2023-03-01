rust 前进太快 有必要自动化项目的维护

社区构建了很多工具
`rustup`
~~~shell

    $ rustup self update
    $ rustup update
~~~

`cargo-outdated` 可用来发现一些老的依赖 可以通过
cargo update 更新这些老的版本

安装 > cargo install --locked cargo-outdated
使用 > cargo outdated

`cargo-audit` 项目的依赖并不是总能升级到最新版的 有时候不得不保持使用旧版
该工具可助我们发现升级后的依赖是否会导致一些vulnerability

安装
> cargo install -f cargo-audit

使用
> cargo audit
