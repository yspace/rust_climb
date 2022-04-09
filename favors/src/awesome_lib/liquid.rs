pub fn run() {
    let template = liquid::ParserBuilder::with_stdlib()
        .build()
        .unwrap()
        .parse("Liquid! {{num | minus: 2}}")
        .unwrap();

    let mut globals = liquid::object!({
        "num": 4f64
    });

    let output = template.render(&globals).unwrap();
    println!("{}", output);
    assert_eq!(output, "Liquid! 2".to_string());
}
