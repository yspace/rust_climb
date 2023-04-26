use once_cell::sync::OnceCell;
use std::sync::Mutex;
// once-cell 和lazy type 正在进标准库的过程中 

static LOG_FILE: OnceCell<Mutex<String>> = OnceCell::new();

/*
“construct on first use” idiom
*/
fn ensure_log_file() -> &'static Mutex<String> {
    LOG_FILE.get_or_init(|| Mutex::new(String::new()))
}

pub fn get_log_file() -> String {
    ensure_log_file().lock().unwrap().clone()
}

pub fn set_log_file(file: String) {
    *ensure_log_file().lock().unwrap() = file;
}

mod _lazy {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    
    /*
    
    even simpler, removing the need for a separate ensure_log_file() function:


     */

    static LOG_FILE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

    pub fn get_log_file() -> String {
        LOG_FILE.lock().unwrap().clone()
    }

    pub fn set_log_file(file: String) {
        // call lock() directly on LOG_FILE. It achieves this by implementing Deref
        /*
         Lazy<T> wraps a OnceCell<T> and implements a Deref<T> that returns 
         self.once_cell.get_or_init(|| self.init.take().unwrap()),
          where init is the closure passed to Lazy::new(). 
         * 
         * The Lazy version still uses the construct on first use idiom, it’s just hidden behind the magic of Deref. 
         * 
         * 但应注意  LOG_FILE 的真实类型 使用时先解引用
         */
        *LOG_FILE.lock().unwrap() = file;
    }
}
