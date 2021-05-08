// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    // a string literal is a slice
    string_slice("blue");
    // trimming a string literal returns a slice
    string_slice("  hello there ".trim());
    // this is explict syntax for taking a slice
    string_slice(&String::from("abc")[0..1]);
    // explicit conversion of literal to String
    string("red".to_string());
    // explicit contstruction from literal
    string(String::from("hi"));
    // creates owned data from borrowed data by cloning the hardcoded string
    string("rust is fun!".to_owned());
    // into will convert to the required type so both will work
    string("nice weather".into());
    string_slice("nice weather".into());
    // I guess format! macro returns String
    string(format!("Interpolation {}", "Station"));
    // exlicit conversion again.
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    // to_lowercase is mutating the data so can't return a slice of the literal
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
