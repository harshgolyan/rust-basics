
fn main() {
    let is_true: bool = true;

    if is_true {
        println!("It is true");
    }
    else {
        println!("It is false");
    }
    
    //loops
    let sentence: String = String::from("my name is harsh golyan");
    let first_word: String = get_first_word(sentence); 
    println!("First word is: {}", first_word);
}

//loops

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for ch in sentence.chars() {
        if ch == ' ' {
            break;
        }
        ans.push(ch);
    }
    return ans;
}