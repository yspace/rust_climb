
// @see https://stackoverflow.com/questions/72175469/how-do-i-call-a-boxdyn-fnonce-stored-inside-a-struct-inside-an-enum/72176378

/* 



FnOnce can only be called once, so it needs to be consumed. 
If it's consumed, then the ClosureContainer must also be consumed (otherwise it's just an invalid struct). 

*/

pub struct ClosureContainer {
    closure: Box<dyn FnOnce(i32) -> i32>,
}

pub enum MathOperation {
    DoNothing,
    RunOperation(ClosureContainer),
}

impl MathOperation {
    pub fn doMath(self, input: i32) -> i32 {
        match self {
            MathOperation::DoNothing => input,
            MathOperation::RunOperation(closureContainer) => (closureContainer.closure)(input),
        }
    }
}

#[test]
fn main() {
    let f = Box::new(move |input: i32| 4 * input);
    let closureContainer = ClosureContainer { closure: f };
    let operation = MathOperation::RunOperation(closureContainer);
    println!("{}", operation.doMath(5));
}