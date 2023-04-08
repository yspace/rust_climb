
trait Trait{
    // methods
    fn takes_self(self);
    fn take_immut_self(&self);
    fn take_mut_self(&mut self);

    /*
 
    // above methods desugared
    fn takes_self(self: Self);
    fn takes_immut_self(self: &Self);
    fn takes_mut_self(self: &mut Self);
    
     */
}
mod _stdlib{
    // std库中的例子
    trait ToString{
        fn to_string(&self)-> String;
    }
}

#[test]
fn test_method_called_using_dot(){
    let five = 5.to_string();
    assert_eq!(five,"5".to_owned());
}
#[test]
fn test_method_called_using_namespaced(){
    let five = ToString::to_string(&5);
    let five2 = i32::to_string(&5);
    assert_eq!(five,five2);
}