fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut vec[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);
}

// fn main() {
//     let mut x = 1;
//     let y = &x;
//     let z = *y;
//     println!("z is {}", z);
//     x += z;
//     println!("x is {}, z is {}", x, z);
// }

// fn main() {
//     let s = String::from("Hello world");
//     let s_ref = &s;
//     drop(s);
//     println!("{}", s_ref);
// }
