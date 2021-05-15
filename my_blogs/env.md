函数
std::env模块下有如下函数：

args	返回命令行传递的参数
args_os	返回当前进程的参数
current_dir	返回当前的工作目录，该目录保存在PathBuf中
current_exe	返回当前可执行程序的全路径
home_dir	返回当前用户目录的路径
join_paths	添加一些环境变量到PATH这个变量中
remove_var	删除当前运行进程的环境变量（系统中的环境变量不会改变，只改变当前进程的环境变量）
set_current_dir	指定当前可执行程序的工作目录
set_var	将当前进程的环境变量修改为新的值
split_paths	Parses input according to platform conventions for the PATH environment variable.
temp_dir	返回临时文件所存放的目录
var	从当前进程获取环境变量，通过环境变量的key值获取value值
var_os	Fetches the environment variable key from the current process, returning None if the variable isn't set.
vars	Returns an iterator of (variable, value) pairs of strings, for all the environment variables of the current process.
vars_os	Returns an iterator of (variable, value) pairs of OS strings, for all the environment variables of the current process.


因为os参数都是字符串 接下来我们很可能要做的就是转型：
使用 use std::str::FromStr 引入 from_str 功能函数； 