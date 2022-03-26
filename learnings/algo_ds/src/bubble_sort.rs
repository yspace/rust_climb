use std::fmt::Debug;
// o(n^2)
pub fn buble_sort<T: PartialOrd+Debug>(v: &mut [T]){
    for p in 0..v.len(){
        let mut sorted = true;
        for i in 0..(v.len() - 1)-p{
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v) ;
        if sorted {
            return;
        }
    }
}
pub fn buble_sort0<T: PartialOrd>(v: &mut [T]){
    for _ in 0..v.len(){
        for i in 0..v.len() - 1{
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort(){
        let mut v = vec![4,6,1,8,11,12];
        buble_sort(&mut v);
        assert_eq!(v , vec![1,4,6,8,11,12]);
    }
}
