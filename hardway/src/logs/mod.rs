
pub fn run(){
    log4rs::init_file(
        // 注意cargo run 运行路径 cd 到harway目录来
        "./src/logs/log4rs.yaml",
        // "./hardway/src/logs/log4rs.yaml",
     Default::default()).unwrap();
   
    // env_logger::init();
    log::error!("This is an error!"); 
    log::info!("This is info!"); 
    log::warn!("This is a warning!");
  
   
}