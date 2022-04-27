use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let file = file!();
    // let pb = std::path::Path::new(&file)
    // .parent().unwrap()
    // .join("configuration");
    // let f = pb.to_str().unwrap();
    // println!("{}", f);
    // // println!("{}",file!()); 
    //  return Ok(());

    // Panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration."); // We have removed the hard-coded `8000` - it's now coming from our settings!
    println!("{:#?}", configuration);
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
