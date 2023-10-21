static KEY: u8 = 142;

struct XOrString {
    text: String
}

impl XOrString {
    fn from_string(text: &str) -> Self {
        let encrypted_string: String = text.chars().map(|c| (c as u8 ^ KEY) as char).collect();
        XOrString {text: encrypted_string }
    }

    fn decrypt(&self) -> String {
        let decryped_string: String = self.text.chars().map(|c| (c as u8 ^ KEY) as char).collect();
        decryped_string
    }
}

fn main() {
    let encryped_string = XOrString::from_string("This string was encrypted using the most advanced post-quantum encryption system");

    println!("{:?}", encryped_string.text);
    println!("{:?}", encryped_string.decrypt());
}
