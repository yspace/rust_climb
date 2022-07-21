
### 文件路径

NOTE As an implementation detail, std::fs::Path and std::fs::PathBuf are implemented on top of std::ffi::OsStr and std::ffi::OsString, respec- tively. This means that Path and PathBuf are not guaranteed to be UTF-8 compliant.