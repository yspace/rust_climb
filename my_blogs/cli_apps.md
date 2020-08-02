##  命令行应用开发知识点

### 流重定向

redirecting the STDOUT and STDERR streams to
separate files:
> cargo run "woof" 1> stdout.txt 2> stderr.txt



### arguments 应用参数

- Binary Flags (aka Switches) 
    binary flags, also known as toggles
    or switches.
    
    long and short version of the flag 
    flag 有长和短形式
    
    When a flag has the bool type, its values are determined by the
    presence of it. If the flag is not present, it will be set to false and vice versa.

- options
    There are other types of arguments called options, which can take a
    value (e.g., --value myvalue ),
    
    
## 颜色输出：

The colored crate defines a Colorize trait, which is implemented on a
&str and String. This trait provides various chainable coloring functions:
-  Coloring the text: red(), green(), blue(), etc.
-  Coloring the background: on_red() (i.e., text on red
background), on_green(), on_blue(), etc.
- Brighter version: bright_red(), on_bright_green(),
etc.
- Styling: bold(), underline(), italic(), etc.    