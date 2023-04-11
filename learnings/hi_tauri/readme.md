
官方步骤出错
cargo install tauri-cli --version ^1.0.0-beta

google到需要从github上安装：
> cargo install tauri-cli --git https://github.com/tauri-apps/tauri --branch next

## nvm 
命令行找不到nvm 
~~~shell
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
~~~
先下载安装


## 杂相

https://users.rust-lang.org/t/global-mutable-variables-in-rust/77056/4

~~~rust

struct Connection;
struct DbConnection {
  db: Mutex<Option<Connection>>,
}

#[tauri::command]
fn connect(connection: State<DbConnection>) {
  // initialize the connection, mutating the state with interior mutability
  *connection.db.lock().unwrap() = Some(Connection {});
}

#[tauri::command]
fn storage_insert(key: u64, value: String, storage: State<Storage>) {
  // mutate the storage behind the Mutex
  storage.store.lock().unwrap().insert(key, value);
}

tauri::Builder::default()
  .manage(Storage { store: Default::default() })
  .manage(DbConnection { db: Default::default() })
  .invoke_handler(tauri::generate_handler![connect, storage_insert])
  // on an actual app, remove the string argument
  .run(tauri::generate_context!("test/fixture/src-tauri/tauri.conf.json"))
  .expect("error while running tauri application");

~~~