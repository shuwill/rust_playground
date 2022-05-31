pub mod guess_game {

    use std::io;
    use std::cmp::Ordering;
    use rand::Rng;

    pub fn guessing_number() {
        println!("Gusse the number!");

        let secret_number = rand::thread_rng().gen_range(1..101);
        //println!("The secret number is {}.", secret_number);

        let mut count = 0;
        loop {
            println!("Please input your guess.");
            count = count + 1;

            /*
            String::new()语法表明new是String类型中的一个关联函数，rust针对类型本身定义关联函数；
            而不会针对String的某个特定示例，类似于Java中的静态方法
            */
            let mut guess = String::new();

            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");

            /*
            rust有一个静态强类型系统，同时他还拥有自动进行类型推导的能力；
            rust允许使用同名的新变量来隐藏（shadow）旧变量的值，这个特性通常被用于在需要转换值类型的场景中
             */
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(err) => {
                    println!("You must type a number {}", err);
                    continue;
                }
            };
            println!("Your guessd: {}", guess);

            /*
            cmp方法能够为任何可比较的值类型计算出他们比较后的结果，
            cmp方法会返回引入作用域的Ordering枚举类型的变体
            match表达式有数个分支（arm）组成，每个分支都包含一个用于匹配的模式，以及匹配成功后要执行的相应的代码
             */
            match guess.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!!");
                }
                Ordering::Greater => {
                    println!("Too big");
                }
                Ordering::Equal => {
                    println!("You win, you guess {} count", count);
                    break;
                }
            }
        }
    }
}
