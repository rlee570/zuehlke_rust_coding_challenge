/*
    Below are 10 functions with descriptions along with tests for each function run the tests
    and write the functions. Once you have completed all the functions and all the tests move on
    to the second challenge it is a little more involved and will use what you've learned here
    (hopefully). These challenges aren't meant to be especially difficult but to ease you into
    the language. Uncomment the functions and the test for each one you wish to do. The triple
    forward slashes are documents and not comments they explain what needs to be done and you
    don't need to uncomment them.
*/

///Write a function that computes the sum of the numbers in a given list(Vectors in rust) using any kind of loop or recursion.
fn sum(values: Vec<i32>) -> i32 {


}

///Write a function that returns only the even numbers
fn even(values: Vec<i32>) -> Vec<i32> {


}

///Write a function that combines two lists such that if we have two lists 1,2,3 and 4,5,6
///it becomes 1,4,2,5,3,6
//fn combine(list_one: Vec<usize>, list_two: Vec<usize>) -> Vec<usize> {
//
//
//}

///Return a String based on the age given as an input to the function
///baby  -> 0-1
///child -> 2-12
///teenager -> 13-19
///adult -> 20 +
//fn age_parser(age: i8) -> &'static str {
//
//
//}

///Check a string then return nothing if it is empty otherwise returns
///the String plus "Hi my name is". Return type is key here.
//fn name_checker(name: &str) -> Result<String, &'static str> {
//
//}

///Given some inputs return a tuple to represent the inputs.
//fn tuple_builder(id: i32, name: &str, age: i8) -> (i32, &str, i8) {
//
//
//}

///Construct a struct with four fields id,country,country_code and currency.
///Add a new function that constructs a country and a function which given an input returns
///a string formatted to have all its fields separated by spaces
///i.e id country country_code currency
//struct Country {
//
//}
//
//impl Country {
//    pub fn new() -> Country {
//
//    }
//
//    pub fn formatted_string(&self) -> String {
//
//    }
//}

///Return a function that adds 3 numbers together
//fn function_builder() -> fn(i32, i32, i32) -> i32 {
//
//}

///Fix the commented out function below.
//fn borrow_checking() -> String{
//    let x = String::from("Hello ");
//    let mut y = &mut x;
//    let z = &y;
//
//    let mut j = String::from("World");
//    let k = j;
//    let l = j;
//
//    z+l
//}

///Make an add,subtract,multiply and divide macro.
///https://doc.rust-lang.org/rust-by-example/macros.html
//macro_rules! calculate {
//
//}

mod tests {
    ///Uncomment tests as you go.
    ///Import the functions as needed this is the full list of all of them
    ///use crate::challenge1::challenge_one::{age_parser, combine, even, function_builder, name_checker, sum, tuple_builder, Country, borrow_checking};
    use crate::challenge1::challenge_one::{sum};
    #[cfg(test)]
    #[test]
    fn should_sum_values() {
        let expected = 100;
        let values = vec![10, 20, 30, 40];
        let actual = sum(values);
        assert_eq!(actual,expected)
    }

    #[test]
    fn should_return_even_values() {
        let expected = vec![2, 4, 8, 6];
        let values = vec![2, 4, 5, 7, 7, 9, 103, 8, 6, 77];
        let actual = even(values);
        assert_eq!(actual,expected)
    }
//
//    #[test]
//    fn should_combine_two_lists() {
//        let expected = vec![1, 4, 2, 5, 3, 6];
//        let list_one = vec![1, 2, 3];
//        let list_two = vec![4, 5, 6];
//        let actual = combine(list_one, list_two);
//        assert_eq!(actual,expected)
//    }
//
//    #[test]
//    fn should_combine_two_unequal_lists() {
//        let expected = vec![1, 4, 2, 5, 3, 6, 7];
//        let list_one = vec![1, 2, 3];
//        let list_two = vec![4, 5, 6, 7];
//        let actual = combine(list_one, list_two);
//        assert_eq!(actual, expected)
//    }
//
//    #[test]
//    fn should_correctly_identify_baby_based_on_age() {
//        let expected = "adult";
//        let age = 0;
//        let actual = age_parser(age);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_correctly_identify_child_based_on_age() {
//        let expected = "child";
//        let age = 7;
//        let actual = age_parser(age);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_correctly_identify_teenager_based_on_age() {
//        let expected = "teenager";
//        let age = 15;
//        let actual = age_parser(age);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_correctly_identify_adult_based_on_age() {
//        let expected = "adult";
//        let age = 25;
//        let actual = age_parser(age);
//        assert_eq!(actual, expected);
//    }
//
//    #[test]
//    fn should_return_greeting_when_given_a_valid_name() {
//        let expected = "Hi my name is Sínead";
//        let name = "Sínead";
//        let actual = name_checker(name).ok().unwrap();
//        assert_eq!(actual, expected)
//    }
//
//    #[test]
//    fn should_not_return_anything_from_empty_string() {
//        let expected = "Not a valid name input";
//        let name = "";
//        let actual = name_checker(name).err().unwrap();
//        assert_eq!(actual, expected)
//    }
//
//    #[test]
//    fn should_return_a_valid_tuple() {
//        let expected = (1, "Siobhan", 23);
//        let id = 1;
//        let age: i8 = 23;
//        let name = "Siobhan";
//        let actual = tuple_builder(id, name, age);
//        assert_eq!(actual, expected)
//    }
//
//    #[test]
//    fn should_return_a_formatted_output_string_from_a_struct() {
//        let expected = "1 Singapore +65 SGD";
//        let country = Country::new(1, "Singapore", "+65", "SGD");
//        let actual = country.formatted_string();
//        assert_eq!(actual, expected)
//    }

//    #[test]
//    fn should_return_a_function() {
//        let expected = 6;
//        let actual = function_builder()(1, 2, 3);
//        assert_eq!(actual, expected)
//    }
//
//    #[test]
//    fn should_not_borrow_mutable_variable_twice(){
//        let expected = "Hello World";
//        let actual = borrow_checking();
//        assert_eq!(actual,expected)
//    }

//    #[test]
//    fn should_use_a_macro_to_add_subtract_multiply_and_divide(){
//        let expected = 4.0;
//        calculate!(add x,2.0+2.0);
//        assert_eq!(x,expected);
//        calculate!(subtract x,8.0-4.0);
//        assert_eq!(x,expected);
//        calculate!(multiply x,2.0*2.0);
//        assert_eq!(x,expected);
//        calculate!(divide x, 8.0/2.0);
//        assert_eq!(x,expected)
//    }
}
