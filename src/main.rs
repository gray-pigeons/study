// use num::complex::Complex;
use std::fmt::Debug;

fn main() {
    // println!("complex_num分支被创建");
    // println!("Hello, world!");

    // let a = Complex {re:2.1,im:-1.2};
    // let b = Complex:new Complex(11.1,22.2);
    // let result = a +b;

    // println!("{}+{}i",result.re,result.im);

    println!("13.14取整={}", 13.14_f32.round());

    let x1: &str = "中";
    println!("x1占用的{}字节的内存大小", std::mem::size_of_val(x1));

    let (x, y) = (3, 2);
    add_with_extra(x, y);
    block();
    let x2 = plus_or_minus(5);
    println!("{}", x2);
    report("1111");

    let s = String::from("hello");
    takes_ownership(s);

    // s 的值移动到函数里 ...,被借用了，所有权被转移了
    // ... 所以到这里不再有效
    // println!("{}", s);

    let s1 = "hello";
    takes_ownership2(s1);

    println!("{}", s1);

    let x = 5;
    makes_copy(x);

    println!("{}", x);

    let s2 = gives_ownership();
    println!("{}", s2);

    let s3 = String::from("hello");

    let _s4 = takes_and_gives_back(s3);

    println!("{}", _s4);

    let mut s5 = String::from("hello ");

    change(&mut s5);
}

fn change(str: &mut String) {
    str.push_str("11111")
}

fn takes_and_gives_back(some_string: String) -> String {
    let str = String::from("world");
    str + &some_string
}

fn gives_ownership() -> String {
    let some_string = String::from("hello world");
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership2(s1: &str) {
    println!("{}", s1);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}

fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

fn add_with_extra(x: i32, y: i32) -> i32 {
    let _a = 8;
    let _b: Vec<f64> = Vec::new();
    let (a, _c) = ("hi", false);
    println!("{}", a);

    //错误的，let 是语句，不是表达式，因此它不返回值，也就不能给其它变量赋值
    // let b = (let a = 8);

    let x = x + 1; //语句
    let y = y + 5;
    println!("x+y不带分号就是表达式,能返回相加的值,带了分号就是一条语句,要加return才有返回值");
    x + y //表达式,不带分号
}

fn block() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y);
}

// #[warn(dead_code)]
// struct Range1 {
//     width: i32,
//     height: i32,
// }

// #[warn(dead_code)]
// impl Range1 {
//     fn add(self: &Range1) -> i32 {
//         self.width + self.height
//     }
// }
