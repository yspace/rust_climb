mod append{
    #[test]
    fn run(){
        let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec1.append(&mut vec2);
    println!("vec1: {:?}", vec1);
    println!("vec2: {:?}", vec2);
    // Output:
    //vec1: [1, 2, 3, 4, 5, 6]
    //vec2: []
    }
}

mod extend{
    fn run(){
        let mut vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        vec1.extend(&vec2);
        println!("vec1: {:?}", vec1);
        println!("vec2: {:?}", vec2);
        // Output:
        // vec1: [1, 2, 3, 4, 5, 6]
        // vec2: [4, 5, 6]
    }
}

mod concat{
    fn run(){
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        let vec3 = vec![vec1, vec2].concat();
        println!("vec3: {:?}", vec3);
        // Output:
        // vec1: [1, 2, 3]
        // vec2: [4, 5, 6]
        // vec3: [1, 2, 3, 4, 5, 6]
    }
}