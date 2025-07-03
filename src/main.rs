mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod m5_lifetimes;
mod m6_patterns;

const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {

    println!("Welcome to our course on {}", OUR_COURSE);

    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // For loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
        } else {
            println!("{}", i);
        }
    }

    // Mutation
    let mut z: i32 = 5;
    print!("z was {} ", z);
    z = 10;
    println!("but is now {}", z);

    let freezing_temp: f64 = -2.4;
    println!("freezing_temp is {}", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("is_zero_remainder is {}", is_zero_remainder);

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji_char: char = '😎';
    println!("emoji_char is {}", emoji_char);

    let my_floats: [f32; 10] = [0.0; 10];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    println!("my_floats_new is {:?}", my_floats_new);

    let name: &str = "Rupa";
    println!("name is {:?}", name);

    let dynamic_name: String = String::from("Rupa Netha");
    println!("dynamic name is {:?}", dynamic_name);

    let str_slice: &str = &dynamic_name[0..4];
    println!("str_slice is {:?}", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('.');
    println!("chars is {:?}", chars);

    let removed_char: char = chars.pop().unwrap();
    println!("removed_char is {:?}", removed_char);
    println!("chars is {:?}", chars);

    //chars.iter().for_each(|c| print!("{}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    dbg!(&chars_again);

    let collected: String = chars_again.iter().collect();
    dbg!(collected);

    for c in chars_again  {
        print!("{}", c);
        if c == 'o' {
            println!(", world!");
        }
    }

    // Closures
    let num: i32 = 5;
    let add_num = |x :i32| x + num;
    let new_num: i32 = add_num(7);
    dbg!(new_num);

    // Number literals
    println!("Big number is {}", 98_22_000);
    println!("Hex is {}",0xff);
    println!("Octal {}", 0o77);
    println!("Binary is {}", 0b1111_0000);
    println!("Bytes 'A' is {}", b'A');

    // Raw String Literal
    let text: &str = r#"{"message" : "Rust is awesome"}"#;
    dbg!(text);

    // Binary
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_0101_1010;
    println!("a's value is {}", a);
    println!("b's value is {}", b);

    println!("a in binary {:08b}", a);
    println!("b in binary {:08b}", b);

    // Logic Gates
    println!("AND {:08b}", a & b);
    println!("OR {:08b}", a | b);
    println!("XOR {:08b}", a ^ b);
    println!("NOT {:08b}", !a);

    // Bitwise Operations
    println!("a << 1 {:08b}", a << 1);
    println!("a << 1 {}", a << 1);
    println!("a >> 1 {:08b}", a >> 1);
    println!("a >> 1 {}", a >> 1);

    // Little Endian or Big Endian
    let n: u16 = 0x1234;
    println!("n is: {:?}", n);

    let big_endian = n.to_be_bytes(); //solana
    let little_endian = n.to_le_bytes();

    println!("n in big endian: {:02X}{:02X}", big_endian[0], big_endian[1]);
    println!("n in little_endian: {:02X}{:02X}", little_endian[0], little_endian[1]);


   
}
 