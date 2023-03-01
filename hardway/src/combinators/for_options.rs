fn option_unwrap_or() {
    let _port = std::env::var("PORT").ok().unwrap_or(String::from("8080"));
}


fn use_case(){
    let default = Some(8080) ;
    let action_option_is_some = |port: i32|->Option<i32>{
        println!("port: {}",port);
        // 这里类型可以不一样哦
        Some(port)
    };
    let action_option_is_none = ||{
        Some(8080)
    };

    let port = None;
    port.or(default)
    .and_then(action_option_is_some)
    .or_else(action_option_is_none) 
    // .as_ref() & .as_mut()
    ;

}