
结构体中的成员类型 应该是类型 还是类型的引用

引用涉及生命周期问题

使用引用的两点考虑：
- 在结构体之外 还被另外的独立的使用到了
- 当类型比较大时 考虑使用引用吧 （移动｜ 复制太贵了？）


### 结构体
Structs are types because they govern the interpretation of a few bytes in memory