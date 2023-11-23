// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.

fn main() {
    let my_crab: &str = "crab";
    let my_gopher: &str = "gopher";
    let my_snake: &str = "snake";
    // &'static is essentially a global variable
    let my_result1: &'static str;
    let my_result2: &'static str;
    let my_result3: &'static str;
    my_result1 = animal_habitat(my_crab);
    my_result2 = animal_habitat(my_gopher);
    my_result3 = animal_habitat(my_snake);
    println!("{}", my_result1);
    println!("{}", my_result2);
    println!("{}", my_result3);
}

pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        99
    };

    // DO NOT CHANGE THIS STATEMENT BELOW
    let habitat = if identifier == 1 {
        "Beach"
    } else if identifier == 2 {
        "Burrow"
    } else if identifier == 3 {
        "Desert"
    } else {
        "Unknown"
    };

    habitat
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
