use std::io;

fn main() {
    println!("請猜測一個數字！");

    println!("請輸入你的猜測數字。");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess);

    println!("你的猜測數字：{guess}");
}
