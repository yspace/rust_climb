该目录拷贝自abs_admin rbatis 官方推荐的例子

ServiceContext等价与yii中的Application
其他服务类 等价与Application-Component 

注意有个ServiceLocator 服务定位 DI 依赖注入等概念可以对比理解

总之 服务 ``service`` 的目的就是提供给其他业务代码的基础服务类
经常作为 依赖传递下去 但如果使用了全局共享(global state 全局状态)的方式
这些应用服务类是作为隐式依赖 不会出现在业务方法的签名中（方法的参数 就是其依赖 经常是 Context|Application|Config|XxxService）

在 abs_admin 项目中服务类充当 java世界SOA｜微服务 的业务处理类
在传统的MVC模式 上添加service层 