use walkdir::{WalkDir, DirEntry} ;
// 还有个异步版本的 async-walkdir

fn main() {
  for entry in WalkDir::new("./") {
    println!("{}", entry.unwrap().path().display());
  }
  // 遍历的项 有些可能因为权限或者正在运行的缘故无法查看 所以只选择ok的项
//   for entry in WalkDir::new("./")
//   .into_iter()
//   .filter_map(|e|{
//     e.ok()
//   }){
//     println!("{}", entry.path().display())
//   }

  WalkDir::new("./")
  .into_iter()
  .filter_entry(|e| is_hidden(e))
  .filter_map(|e|e.ok())
  .for_each(|e|{
    println!("{}", e.path().display());
  });
}
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}