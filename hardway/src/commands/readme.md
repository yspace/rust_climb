

https://github.com/JoshMcguigan/bubble-shell
https://www.joshmcguigan.com/blog/build-your-own-shell-rust/

https://github.com/codecrafters-io/build-your-own-x


https://stackoverflow.com/questions/40836973/unable-to-use-stdprocesscommand-no-such-file-or-directory
~~~rust

Command::new("/usr/bin/ssh")
    .args(&["-i", "/tmp/.ssh/25.pem", "ubuntu@ip"])
    .spawn()

~~~