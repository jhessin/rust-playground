/* Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

struct IgpayAtinlay {
  data: String,
}

impl IgpayAtinlay {
  fn from(instring: &str) -> IgpayAtinlay {
    let vowels = "AEIOUaeiou";
    let mut data = String::new();

    for word in instring.to_ascii_lowercase().split_whitespace() {
      if let Some(letter) = word.chars().next() {
        if vowels.contains(letter) {
          data.push_str(word);
          data.push_str("-hay ")
        } else {
          if let Some(slice) = word.get(1..) {
            data.push_str(slice);
            data.push('-');
            data.push(letter);
            data.push_str("ay ");
          }
        }
      }
    }

    IgpayAtinlay { data }
  }

  fn print(&self) {
    println!("{}", self.data);
  }
}

pub fn main() {
  IgpayAtinlay::from("This is piglatin at it's finest").print();
  IgpayAtinlay::from("Hey! Someone took my apple!").print();
  IgpayAtinlay::from("I got the first apple!").print();
}
