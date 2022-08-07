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

    let yellow1 = FlowerColor::Yellow;
    let red = FlowerColor::Red;
    let white = FlowerColor::White;
    print_flower_color(yellow1);
    print_flower_color(red);
    print_flower_color(white);

    let flower = Flower {
        white_flower: FlowerColor::White,
        red_flower: FlowerColor::Red,
        yellow_flower: FlowerColor::Yellow,
    };
    println!("{:?}", &flower);
    let f1 = Flower::flower_new(flower.white_flower, flower.red_flower, flower.yellow_flower);
    println!("{:#?}", &f1);

    let f2 = Flower::flower_new_one(f1);
    println!("{:#?}", &f2);

    let card_quit = Card::Quit;
    let card_message = Card::CardMessage(String::from("消息"), 22);
    let card_move = Card::CardMove { x: (23), y: (33) };
    let card_color = Card::CardColor(FlowerColor::Red);

    println!("{:#?}", &card_quit);
    println!("{:#?}", &card_message);
    println!("{:#?}", &card_move);
    println!("{:#?}", &card_color);
}

enum Card {
    CardMessage(String, i32),
    CardColor(FlowerColor),
    CardMove { x: i32, y: i32 },
    Quit,
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CardMessage(arg0, arg1) => f
                .debug_tuple("CardMessage")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::CardColor(arg0) => f.debug_tuple("CardColor").field(arg0).finish(),
            Self::CardMove { x, y } => f
                .debug_struct("CardMove")
                .field("x", x)
                .field("y", y)
                .finish(),
            Self::Quit => write!(f, "Quit"),
        }
    }
}

#[derive(Debug)]
struct Flower {
    white_flower: FlowerColor,
    red_flower: FlowerColor,
    yellow_flower: FlowerColor,
}

impl Flower {
    fn flower_new(
        white_flower: FlowerColor,
        red_flower: FlowerColor,
        yellow_flower: FlowerColor,
    ) -> Self {
        Self {
            white_flower,
            red_flower,
            yellow_flower,
        }
    }

    fn flower_new_one(color: Flower) -> Flower {
        color
    }
}

fn print_flower_color(color: FlowerColor) {
    println!("{:?}", &color);
}

#[derive(Debug)]
enum FlowerColor {
    White,
    Red,
    Yellow,
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
