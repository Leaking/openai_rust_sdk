const OPENAI_HOST: &str = "api.openai.com";
mod OPENAI_PATH {
    const LIST_MODEL: &str = "v1/models";
    const RETRIEVE_MODEL: &str = "v1/models/{model}";
    const CREATE_COMPLETION: &str = "v1/completions";
    const CREATE_CHAT_COMPLETION: &str = "v1/chat/completions";
    const CREATE_EDIT: &str = "v1/edits";
}

fn main2() {
    println!("Hello, world!");
}

fn main() {
    let a = "abc";
    let mut b = String::from("value");
    let c: &str = &b[0..3];
    b.to_string();
    println!("Hello, world! {}, {}, {}", a, b, c);
}
