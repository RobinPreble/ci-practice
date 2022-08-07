fn main() {
    println!("Hello World!");
} 
// this should only be in the test branch
fn return_word(word: &str) -> &str {
    word
}
fn return_upper(word: &str) -> String {
        word.to_uppercase()
    }

#[cfg(test)]
mod test {
    use crate::{return_word, return_upper};

    #[test]
    fn tests () {
        assert_eq!(return_upper("greg"), "greg")
    }

}