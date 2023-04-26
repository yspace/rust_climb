
https://doc.rust-lang.org/reference/procedural-macros.html

https://stackoverflow.com/questions/52585719/how-do-i-create-a-proc-macro-attribute


> $ rg -c proc_macro_attribute

~~~toml

[lib]
crate_type = ["proc-macro"]

~~~

上面这个是不是老语法


下面这个是现在使用的？
[lib]
proc-macro = true