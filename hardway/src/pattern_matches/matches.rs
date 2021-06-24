pub fn run(){

    let dire = Direction::South ;
    match dire {
        Direction::East => println!("East") ,
        Direction::North | Direction::South => {
            println!("South or North") ;
        },
        _ => println!("West") ,
    }

    // 
    as_expression() ;
    //
    de_construct();
}

enum Direction{
    East ,
    West,
    North,
    South,
}

fn as_expression(){
    let d_west = Direction::West ;
    let d_str = match d_west {
        Direction::East => "East" ,
        Direction::North | Direction::South => {
            panic!("South or North") ;
        },
        _ => "West" ,
    } ;

    println!("{}", d_str) ;
}

fn de_construct(){
    enum Action{
        Say(String) ,
        MoveTo(i32, i32) ,
        ChangeColorRGB(u16,u16,u16) ,
    }

    //
    let action = Action::Say("Hello Rust".to_string() );
    match action{
        Action::Say(s) => {
            println!("{}", s) ;
        },
        Action::MoveTo(x,y) => {
            println!("point from (0,0) move to ({},{})", x, y) ;
        },
        Action::ChangeColorRGB(r,g, _) => {
            println!("change color into : (r:{}, g:{}, b:0) b is ignored",r, g) ;
        },

    }
}