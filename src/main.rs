// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

const CONST_PI:f32 = 3.14;

fn main() {
    day2()
}

fn day2() {
    let a = 1;
    println!("{a}{CONST_PI}");
    let mut a = 2;
    println!("{a}");
    a = 123;
    println!("{a}");
}

// fn day1() {
//     let rand_num = rand::thread_rng().gen_range(1..=100);

//     loop {
//         let mut input_num: String = String::new();
//         println!("请输入 -> {rand_num}-{input_num}");

//         match io::stdin().read_line(&mut input_num) {
//             Ok(val) => val,
//             Err(_) => continue,
//         };

//         let input_num: u32 = match input_num.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         match input_num.cmp(&rand_num) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }