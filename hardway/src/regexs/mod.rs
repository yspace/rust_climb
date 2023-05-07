use regex::Regex;

#[test]
fn foo(){
    // @see https://dev.to/carlyraejepsenstan/regular-expressions-in-rust-5c1c
    //  `r` 用来应对反斜杠的问题
    // let regex = Regex::new(r"\d{3}").unwrap();
    // let a: &str = "123bbasfsdf23asd2021-06-17";

    let a: &str = "123bbasfsdf23asd2021-06-17";
    for cap in Regex::new(r"\d{1,3}").unwrap().find_iter(a) {
        // println!("{:#?}", cap);
        println!("{:#?}", cap.as_str());
    }

    let my_captures: Vec<&str> = (Regex::new(r"\d{1,3}")
    .unwrap()
    .find_iter(a)
    // 不做map 不可以直接收集么？收集成Vec<String>
    .map(|x| x.as_str())
    // 
    .collect());
    println!("{:?}", my_captures);
}