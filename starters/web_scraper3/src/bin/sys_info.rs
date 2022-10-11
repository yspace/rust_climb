
use std::path::Path;
fn main(){
    use sysinfo::{DiskExt, System, SystemExt};

    // let path = "/".to_string();
    let path = std::path::Path::new("downloads");
    let path = path.to_str().unwrap().to_string() ;

    let sys = System::new_all();
  for disk in sys.disks() {
    println!("{:?}", disk) ;
    if Path::new(&path).starts_with(disk.mount_point()) {
    //   return disk.available_space();
      let available_space = disk.available_space(); 
      println!("{}", available_space) ;
    }
  }
}