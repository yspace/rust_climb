pub fn main() {
    let args = env::args().collect ::<Vec<String>>();

    let input  = args.get(1).unwrap();
    let defaut = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&defaut);

    let operations = parse(input) ;
    let path_data = convert(&operations); 
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}

#[derive(Debug,Clone,Copy)]
enum Operation {
    Forwards(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(usize),
}