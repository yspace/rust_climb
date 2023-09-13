
结构体中的成员类型 应该是类型 还是类型的引用

引用涉及生命周期问题

使用引用的两点考虑：
- 在结构体之外 还被另外的独立的使用到了
- 当类型比较大时 考虑使用引用吧 （移动｜ 复制太贵了？）


### 结构体
Structs are types because they govern the interpretation of a few bytes in memory


## FAQ

- [Is it possible to call a parent struct's methods from a child struct?](https://stackoverflow.com/questions/59364133/is-it-possible-to-call-a-parent-structs-methods-from-a-child-struct)

4

The short answer is no.

RefCell owns its inner object. This means it has the only copy of that object, so that it can fully control all access to it and not allow any other access from outside. An object can't exist in RefCell and outside of RefCell at the same time.

Your setup could take an existing Rc<RefCell<BUS>> instead and pass that around. &mut BUS without the wrapper won't do.

The borrow checker can't ensure safety of mutual parent-child relationships. It wants program data structured as trees or DAGs. Otherwise you're forced to use wrapper types or C-like raw pointers.

The borrow checker checks against interfaces, not implementation. If your setter borrows &mut self, that's exclusive borrow of the entire object, and for borrow checking that's the most restrictive and most inflexible option. You will need to peel some layers of abstraction to make this work, e.g. pass RAM down to the CPU. Alternatively, make RAM use Cell<u8> type, so that it can be mutated via shared references.


- [How do I express mutually recursive data structures in safe Rust?](https://stackoverflow.com/questions/36167160/how-do-i-express-mutually-recursive-data-structures-in-safe-rust?noredirect=1&lq=1)


