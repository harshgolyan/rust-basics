

fn main() {
    let mut s1: String = String::from("Hello");
    // let s2: String = s1;
    println!("s1: {}", s1);
    // borrower();
    mutable_borrower(&mut s1);
    println!("s1 after mutation: {}", s1);
}


//borrower

// fn borrower() {
//     let s1: String = String::from("Hello borrower");
//     let s2: &String = &s1; // Borrowing
//     println!("s2: {}", s2);
// }

fn mutable_borrower(s: &mut String) {
    s.push_str(" world"); 
}