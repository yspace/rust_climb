pub enum Status{
    Queued,
    Running,
    Failed,

}

pub fn print_status(status: Status) {
    match status {
        Status::Queued => println!("queued"),
        Status::Running => println!("running"),
        Status::Failed => println!("failed"),
    }
}

pub fn run(){
    let status = Status::Running;
    print_status(status) ;
}