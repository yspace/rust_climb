Rust type conversions fall into three categories:


    manual: user-defined type conversions provided by implementing the From and Into traits
    手动 通过实现From Into traits来提供类型转换

    semi-automatic: explicit casts between values using the as keyword
    半自动化 值间显式使用`as`

    automatic: implicit coercion into a new type.
    自动化 隐式协变到新类型