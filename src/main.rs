// use num::complex::Complex;
use std::{fmt::Debug, io};

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

    let some_number = Some(6);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = Option::None;
    let _absent_string: Option<u8> = Option::None;

    println!("{:?}", &some_number);
    println!("{:?}", &_some_string);
    println!("{:?}", &_absent_number);
    println!("{:?}", &_absent_string);

    let arr = [1, 2, 3, 4, 5];
    println!("请输入一个数组索引0--{}", arr.len() - 1);

    let mut index = String::new();
    //读取控制台输入
    io::stdin().read_line(&mut index).expect("读取失败");

    let index: usize = index.trim().parse().expect("输入的索引不是一个数字");
    // index = match &index {
    //     // &index => print!("1111"),
    // };
    if index >= arr.len() || index < arr.len() - arr.len() {
        println!("输入的索引超限");
        return;
    }
    let element = arr[index];
    println!("输入的有效索引是{0},输出的结果是{1}", index, element);

    let _some_number = Some(6);
    let _some_string = Some("a string aaaaaaa");
    let _absent_number: Option<i32> = Option::None;

    let slice = &arr[1..element];
    println!("数组的切片{:?}", &slice);
    // assert_eq!(
    //     slice,
    //     &[2, 3, 4, 5],
    //     "这是额外的报错信息,如果两者不完全相等，则进程就会报错退出"
    // );

    let dire1 = Direction::South;
    let dire2 = Direction::East;
    let dire3 = Direction::West;
    let dire4 = Direction::North;
    let dir_arr = [dire1, dire2, dire3, dire4];
    for dire in dir_arr {
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            }
            _ => println!("West"),
        };
    }

    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("s->{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("x->{},y->{}", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "Change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g
                );
            }
        }
    }

    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        9 => println!("nine"),
        _ => (),
    }
    let mut num = 0;
    for mut i in 0..255u8 {
        num += 1;
        i = i & 1;
        match i {
            0 => println!("偶数 ==>{}", num),
            1 => println!("奇数==>{}", num),
            _ => (),
        }
    }

    if let Some(3) = Some(3) {
        println!("three,true")
    }

    if let 3u8 = some_u8_value {
        println!("{}", some_u8_value)
    }

    let vec_arr = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    // vec_arr.iter().filter(|x| x == MyEnum::Foo);
    let v = vec_arr.iter().filter(|x| matches! {x,MyEnum::Foo});
    println!("{:?}", v);

    let five = Option::Some(5);
    println!("{:?}", five);
    let six = plus_one(five);
    let none = plus_one(Option::None);
    println!("{:?}", six);
    println!("{:?}", none);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    //while let一个与  if let 类似的结构是  while let 条件循环，它允许只要模式匹配就一直进行  while 循环。
    //  pop 方法取出动态数组的最后一个元素并返回
    // Some(value) ，如果动态数组是空的，将返回  None ，对于  while 来说，只要  pop 返回  Some
    // 就会一直不停的循环。一旦其返回  None ， while 循环停止。我们可以使用  while let 来弹出
    // 栈中的每一个元素。
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    stack.push(4);
    println!("{}", stack[0]);

    stack.push(5);
    stack.push(6);
    stack.push(7);
    stack.push(8);
    loop {
        if let Some(top) = stack.pop() {
            println!("{:?}", top);
        } else {
            break;
        }
    }

    stack.push(9);
    stack.push(10);
    stack.push(11);
    stack.push(12);

    //迭代器
    for (index, value) in stack.iter().enumerate() {
        println!("{} is at index={}", value, index);
    }

    println!("{}", stack[0]);
    for (_, value) in stack.iter().enumerate() {
        println!("value is ={}", value)
    }

    for num in &stack {
        println!("{}", num);
    }

    let num = vec![stack[0], stack[stack.len() - 1]];
    println!("{:?}", num);
    println!("{:?}", assert!(matches!(stack[0], 0..=10)));

    for i in 1..5 {
        // println!("{}", i);
        match i {
            // 1 => println!("{}", i),
            _ => println!("{}", i),
        }
    }
    println!("------------------------------------------");

    let x = Some(5);
    let _y = 10;
    match x {
        Some(50) => println!("Go to 50"),
        //这里的_y其实是一个全新的变量，并不是指的外面那个_y
        Some(_y) => println!("y={:?}", _y),
        _ => println!("x={:?}", x),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(i) => Option::Some(i + 1),
        Option::None => Option::None,
    }
}
#[derive(Debug)]
enum MyEnum {
    Foo,
    Bar,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter form {:?}!", state);
            25
        }
    }
}

enum Direction {
    East,
    West,
    North,
    South,
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

impl<T> From<T> for Option<T> {
    fn from(v: T) -> Self {
        Self::Some(v)
    }
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
