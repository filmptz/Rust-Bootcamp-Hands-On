fn main() {
    // #1 declare 2 strings
    let string1 = String::from("Hello");
    let string2 = String::from(" World!");
    // #2 call the `concatenated_string` function to concat the `string1` and `string2` by passing as immutable ref
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("concatenated_string is {}", concatenated_string);

}
/**
 * concatenate_strings is used to concat the given two strings
 * @param str1: immutable ref of the given str1
 * @param str2: immutable ref of the given str2
 * @return result: String value
 */

fn concatenate_strings(str1: &String, str2: &String) -> String {
    // #3 declare the `result` as the empty string
    let mut result = String::from("");
    // #4 concat the result with the str1 and str2, respectively
    result.push_str(str1);
    result.push_str(str2);
    // #5 return the `result` as a `String` type
    result
}