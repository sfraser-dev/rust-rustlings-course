// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main (){
    let my_str: &'static str = "   Hi there ";
    println!("{}",trim_me(my_str));
    println!("{}",compose_me("hello"));
    println!("{}",replace_me("hello cars"));
}


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // ???
    // String::from(input.to_string().trim())
    input.trim().to_owned()
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    // ???
    // convert &str to String
    let mut my_heap_string = input.to_owned();
    my_heap_string.push_str(" world!");
    my_heap_string
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    // ???
    // let ballons = String::from("balloons");
    let my_heap_string: String = input.to_owned();
    let my_heap_string_altered: String = str::replace(&my_heap_string, "cars", "balloons");
    my_heap_string_altered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
