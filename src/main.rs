use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut input_num: String = String::new();
        println!("请输入 -> {rand_num}-{input_num}");

        io::stdin().read_line(&mut input_num).expect("用户输入");

        let input_num: u32 = match input_num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input_num.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
