在物理世界中

generic的一个PC例子 比如usb 可以适用于不同的小设备如 摄像头 移动盘 手写板 ... 只要这些设备尊从
usb协议接口 


fn usb<T: USBDevice>(){}

同理。函数可以通过特定类型执行特定的任务 ，generic函数可以通过一批类型（这些类型需要满足泛型约束 满足特定条件 比如usb接口 usb协议支持。通过泛型 可以把实现推迟到以后 先定义一个trait｜interface 即便目前没有实现的struct 只要有这个接口协议 就可以先开工 ，或者同时开工--一批人开发基于约束的上层工作 另一批人根据接口约束去开发实现 前者一般是架构级或者库开发人员他们依赖抽象编程 后者一般是小屁孩 小白或者驱动开发人员 ）执行特定任务

.

Generic programming goals is to improve code reusability and reduce bugs by allowing functions, structures and traits to have their types defined later. In practice it means that an algorithm can be used with multiple differens types, provided that they fulfill the constraints.