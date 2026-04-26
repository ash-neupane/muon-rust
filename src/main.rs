

// fn multiply_numbers() {
//     let x: i32 = 5;  // like const
//     let mut y: i32 = 4;
//     y = 6;

//     let z: i32 = 5;

// }

fn _add(a: i32, b: i32) -> i32 {
    a + b
}



fn main() {
   let s1 = String::from("hello");
   // this moves:
   // let s2 = s1;
   // println!("s1: {}", s1); 
   // println!("s2: {}", s2);

    // this borrows i.e. r points to the stack header of s.
    let s2 = &s1;
    println!("s1: {}, s2: {}", s1, s2);

    // can you do this? yes, immut is a copy
    let r1 = &s1;
    let r2 = r1;
    println!("r1: {}, r2: {}", r1, r2);

    let n = 5;
    let m = n;
    println!("{}", n);
    println!("{}", m);


    // let s = String::from("hi");
    // let r1 = &s;
    // let r2 = &s;
    // println!("r1: {}, r2: {}", r1, r2);

    // let mut v = vec![1, 2, 3];
    // for x in &v {
    //    v.push(*x);
    // }

    let mut s = String::from("hi");
    let r1 = &mut s;
    let r2 = r1;
    r1.push_str("!"); // mut is a move
}

