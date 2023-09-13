struct Operation {
    params: Vec<String>,
    ops: Box<dyn Fn(Vec<String>) -> Vec<String>>,
}

#[test]
fn run() {
    let obj = Operation {
        params: Vec::new(),
        // wrap a closure
        ops: Box::new(|strings| {
            /* do something... */
            strings
        }),
    };

    // call the function or closure inside the Box
    // (obj.ops)(Vec::new()) 
    // FIXME: how can i make this to run  
    let ops = obj.ops;
}
