fn main() {
    let x = (-42.0_f32).sqrt();
    assert_eq!(x, x);
}

fn defensively(){
    let x : f32 = 1.0/ 0.0 ;
    assert!(x.is_infinite())
}