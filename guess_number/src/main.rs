// prelude 预导入
use std::io;
// import commonly used items from the prelude:
// trait 这里没有用到, 是因为只有引入了这个 trait 才能使用里面的方法
// trait 类似于 interface, 里面会定义很多方法
use rand::Rng; 
use std::cmp::Ordering;


// rust中默认所有的变量都是不可变的 immutable

fn main() {
    // ? 为什么这里使用 print!, 运行程序不会在控制台先输出, 而 println! 可以
    println!("猜数!");



    // ? gen_range 是 Rng trait 定义的方法
    // 因为下面进行了比较, 比较的值的类型是 u32, 所以这里类型推断为 u32
    let snum = rand::thread_rng().gen_range(1..101); 
    println!("随机数: {}", snum);


    loop {
        // ? 声明 num 在外部, 会导致出错
        // 关联函数, 相当与静态方法
        // 类型推导
        let mut num = String::new();
        // 因为使用use导入了io的引用, 所以可以直接使用
        // read_line 需要一个可变的字符串引用, 所以使用$mut
        // rust核心竞争力是我们可以比较安全的使用引用
        // read_line -> io::Result, rust中存在很多的Result, 实际上就是一个枚举类型, 枚举类型的值被称为枚举类型的变体
        // Result Ok + 返回值, Err + 失败原因
        // Result 枚举类型上还定义了一系列的方法, expect是其中之一
        // ? expect 的运作方式是 Result 返回的是 Err, expect 会中断函数的执行, 并显示传入的字符串, 如果是Ok, 就返回值
        // let result = io::stdin().read_line(&mut num).expect("无法读取行");
        let result = io::stdin().read_line(&mut num).unwrap();
    
    
        // {} 表示占位符, 会顺序替换成后续传入的变量值
        println!("你猜测的数字是: {}, {}", result, num);
    
        // shadow 隐藏前面的变量
        // parse 方法返回的有多重类型, 需要指定 num 的类型, 才可以编译通过
        // let num: u32 = num.trim().parse().expect("error");
        // 或者可以使用这种写法
        // expect 输入非数字的时候, 程序会崩溃
        let num = num.trim().parse::<u32>().expect("error");
    
        // cmp 返回 Ordering 三种情况
        // match 表达式根据后面的值来决定后续的执行
        match num.cmp(&snum) {
            // 引入 Ordering 枚举类型, 下面三个变体
            Ordering::Greater => println!("too big"), // arm
            Ordering::Less => println!("too small"),
            Ordering::Equal => println!("you win"),
        }

    }

}
