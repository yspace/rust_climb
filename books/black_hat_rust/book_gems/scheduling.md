协作式 抢占式调度

go 使用的是抢占式 task无async和sync的区别 

rust中的运行时使用的是协作式 通过await 告诉运行时需要等io了！
await in code are exactly that. They are indications for the runtime (and compiler), that the
task will take some time awaiting for an I/O operation to complete, and thus the computing power can be used for another task in the meantime.

It has the advantage of being extremely fast. Basically, the developer and the runtime are working together, in harmony, to make the most of the computing power at disposition.

协作是跟runtime协作 还是跟其他异步块协作？^_^ .