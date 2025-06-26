

fn main() {
    let x: i32 = -5; //signed
    let y: u32 = 15; //unsigned
    let z: f32 = 10.00; //float
    println!("x: {}, y: {}, z: {}", x, y, z);

    //boolean
    let is_male: bool = true;
    let is_adult: bool = true;

    if is_male {
        println!("You are male");
    }
    else {
        println!("You are not male");
    }
    if is_male && is_adult {
        println!("You are male and adult");
    }

    //strings
    strings();
}


fn strings() {
    let str = String::from("Hello world :)");
    println!("{}", str);

    print!("{}", str.chars().nth(1).unwrap());
}