//猜数字游戏
use rand::Rng;
use std::cmp::Ordering;
use std::io;
//生成随机数字
fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=100)
}

fn main() {
    println!("猜数字游戏");
    let secret_number = generate_random_number();
    loop {
        println!("请输入一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess_num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("解析错误，请输入正确数字");
                continue;
            }
        };
        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}
