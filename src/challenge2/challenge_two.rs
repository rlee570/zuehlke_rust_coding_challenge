use regex::Regex;
//Regex Crate docs
//https://docs.rs/regex/1.3.1/regex/
//Regex dependancy examples
//https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
/*
  T9 was a common standard for text input on mobile phones. Pressing numeric keys on a keypad from
  1-9 could represent different letters. Your boss has tasked you to create a parser to parse the
  numbers from a keypad into a string from a sequence of numeric inputs. For now you only have to
  worry about alphabetical input and you can assume there will be no incorrect input.
   Mappings:
   1 -> abc
   2 -> def
   3 -> ghi
   4 -> jkl
   5 -> mno
   6 -> pqr
   7 -> stu
   8 -> vwx
   9 -> yz
   Example Input and the expected output:
   1->a
   11->b
   111-> c
   1111-> ca
   3322444444555 -> hello

   IMPORTANT TO NOTE: RUST STRINGS ARE UTF-8 ENCODED INTERNALLY AND ARE NOT INDEXED
*/

fn parse_input(input: &str) -> String {
    //Example usage of regex crate
    let re = Regex::new(r"").unwrap();
    //placeholder
    String::from("")
}

mod tests {
    use crate::challenge2::challenge_two::parse_input;

    #[cfg(test)]
    #[test]
    fn should_output_a_string_of_letters_given_a_string_of_numbers() {
        let numbers = "3322444444555";
        let expected = "hello";
        let actual = parse_input(numbers);
        assert_eq!(expected, actual)
    }

    /*
      After successfully completing your previous parser your boss now wants you to further extend
      it to be able to handle spaces between words. Spaces will be denoted in a string with a 0
      character.
    */
    #[test]
    fn should_output_a_string_of_letters_and_spaces_given_a_string_of_numbers() {
        let numbers = "33224444445550885556664442";
        let expected = "hello world";
        let actual = parse_input(numbers);
        assert_eq!(expected, actual)
    }

    /*
      Your boss has become very angry as he realises your parser can not handle numbers. Quickly
      implement the ability to handle numbers (they will be preceded with an underscore(_))
      in a string before your boss throws the book at you.
    */
    #[test]
    fn should_output_a_string_of_letters_numbers_and_spaces_given_a_string_of_numbers_and_underscores(
    ) {
        let numbers = "332244444455508855566644420_1_2_3";
        let expected = "hello world 123";
        let actual = parse_input(numbers);
        assert_eq!(expected, actual)
    }

    /*
      Your boss has once again entered the room foaming at the mouth there is no capital letters he
      gesticulates towards you. Implement capital letters (they will be preceded by an *) in a
      string before your boss goes postal.
    */
    #[test]
    fn should_output_a_string_of_letters_numbers_caps_and_spaces_given_a_string_of_numbers_asterisks_and_underscores(
    ) {
        let numbers = "*33224444445550*8855566644420_1_2_3";
        let expected = "Hello World 123";
        let actual = parse_input(numbers);
        assert_eq!(expected, actual)
    }

    /*
      Your boss bursts through the door screaming in a frenzy. The most you can make out of the feral
      noises is "ERRORS ERRORS ERRORS". Its probably time to handle errors before this gets anymore
      out of hand. Change the return type to correctly deal with errors. You may want to create a
      separate function if you wish to preserve the original return type function for reference.
    */
//    #[test]
//    fn should_handle_an_erroneous_string_correctly() {
//        let numbers = "Iamnotnumbers";
//        let expected = "Error parsing numbers";
//        let actual = parse_input(numbers).err().unwrap();
//        //let actual = parse_input_error(numbers).err().unwrap();
//        assert_eq!(expected, actual)
//    }
}