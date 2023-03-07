

~~~rust


impl<T, E> Result<T, E> 
    where E: Debug

~~~

注意作为错误的E 有个约束 

不然result上的有些方法就用不了哦！

一般方法找不到 不是因为没有倒入traits 及其实现 就是这种约束 只有类型满足特定条件下 某些方法才可见。

比如上面的Result 再次抽象下 更一般化
~~~

imple<T,E> SomeThing<T,E> 
where E: SomeTrait
{
    fn some_fn_only_visible_when_e_is_some_trait(){}
}


~~~
