
整洁架构 会跟洋葱架构 六边形架构 有些相似性

- [Hex Architecture]()


## 参考
https://manakuro.medium.com/clean-architecture-with-go-bce409427d31

https://github.com/amitshekhariitbhu/go-backend-clean-architecture

https://crosp.net/blog/software-architecture/clean-architecture-part-2-the-clean-architecture/

https://github.com/MSC29/clean-architecture-rust

https://www.codingblocks.net/podcast/clean-architecture-make-your-architecture-scream/

## 目录结构

- domain 里面应该是领域模型   可以参考ddd   同时此处可以有repository的接口定义

- usecase 用例层 推荐做法是每用例一个类（类似 早期mvc中 每个Action一个类那样）
后缀名 XxxInter

- infra 基础结构层 Repository的实现常常出现在此层
  有个adapter 适配器层 也是用来实现接口的 包括仓储接口