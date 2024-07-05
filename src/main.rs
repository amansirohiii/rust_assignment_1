use rand::Rng;
use chrono::{Local, Utc};
use std::fs;
use futures::executor::block_on;
use std::io;

struct User{
    name: String,
    age: u32,
    active: bool
}
enum Shape{
    Circle(f64),
    Rectangle(f64,f64)
}
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let name = String::from("aman");
    println!("name: {name}");
    let char1 = name.chars().nth(0);
    println!("{}", char1.unwrap());

    let x = true;
    if x{
        println!("true")
    }
    else{
        println!("false")
    }

    for i in 1..10000{
        println!("{i}")
    }

    let sentence = String::from("aman!");
    let name = get_first_word(sentence);
    println!("{name}");
    fn get_first_word(sentence: String) -> String {
        let mut ans = String::new();
        for char in sentence.chars() {
            ans.push_str(char.to_string().as_str());
            if char == ' ' {
                break;
            }
        }
        return ans;
    }

    let name = String::from("Aman");
    let user = User{
        name: name,
        age: 20,
        active: true
    };
    println!("{} is {} years old and is {}", user.name, user.age, user.active);
    println!("{:p}", user.name.as_ptr());

    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(5.0, 6.0);
    let area1 = calculate_area(circle);
    let area2 = calculate_area(rectangle);
    println!("Area of Circle: {} and Rectangle: {}",area1, area2);

    let n = rand::thread_rng().gen_range(1..=100);
    println!("{}", n);

    let now = Utc::now();
    let local = Local::now();
    println!("Current Time is: {}", now);
    println!("Current Time is: {}", local);

    let mut s = String::from("Hello");
    calculate_length(&mut s);
    println!("{}", s);

    let mut arr = ["Hello", "Aman", "Sirohi"];
    update_array(&mut arr);
    println!("{:?}", arr);

    let mut v: Vec<i32> = Vec::new();
    let _v2 = Vec::<i32>::new();
    let v3 = vec![1,2,3,4,5];
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v3);

     let mut v = vec![1,2,3,4,5];
    //  print_vector(v);
     println!("{:?}", v);

    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    s2.push_str(" World");
    println!("{}", s2);
    let s3 = &s1;
    println!("{}", s3);

    let file_result = fs::read_to_string("hello.txt");
    match file_result{
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    block_on(hello_world());

    let a = [1, 2, 3, 4, 5];
    for ele in a{
        println!("{}", ele);
    }
    println!("{:?}",a);
    // loop{
    //     println!("{}", 1);
    // }
    let mut n = 0;
    while n < 5 {
        println!("{}", n);
        n +=1;
    }

    let n = 10;
    match n {
        1 => println!("not approved"),
        10 => println!("approved"),
        _ => println!("default")
    };

    match n{
        n if is_even(n) => println!("even"),
        n if !is_even(n) => println!("odd"),
        _ => println!("not applicable")
    };

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("error occured");
    println!("{}", input);
}
fn is_even(n: i32) -> bool{
    n%2==0
}
async fn hello_world() {
    println!("hello, world!");
}

fn print_vector(v1: Vec<i32>){
    println!("{:?}", v1);
}

fn update_array(arr2: &mut [&str; 3]){
    arr2[0]="I'm";
    println!("{:?}", arr2);
}

fn calculate_length(s: &mut String)-> &String{
    (*s).push_str(" World");
    return s;
}

fn calculate_area(shape: Shape)-> f64{
    let ans = match shape{
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Rectangle(length, breadth ) => length*breadth
        };
        ans
}
