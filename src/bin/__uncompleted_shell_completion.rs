

fn main() {
    let word_list = vec!["you've", "been", "completed"];
    let args : Vec<String> = std::env::args().collect();
    let to_complete : &String = &args[2];

    for word in word_list {
        if word.starts_with(to_complete) {
            println!("{}", word);
        }
    }
}
