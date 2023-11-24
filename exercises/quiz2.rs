// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

// I AM NOT DONE

#[derive(Debug, PartialEq, Clone)]
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

fn my_transformer(vec_in: &mut Vec<(String, Command)>) {
    // going to mutate the input vector (in place)
    vec_in.iter_mut().for_each(|x: &mut (String, Command)| {
        // grab the String and Command for this particular tuple element
        let mut _s: String = (x.0).clone();
        let _c: Command = (x.1).clone();
        // perform the Command upon the String
        match _c {
            Command::Uppercase => x.0 = String::from((x.0).as_str().to_uppercase()),
            Command::Trim => x.0 = String::from((x.0).as_str().trim()),
            // grab the Command::Appends variant value
            Command::Append(variant_value) => {
                let mut app_str: String = String::from("");
                // append variant_value "bar"s together
                for _ in 0..variant_value {
                    app_str.push_str("bar");
                }
                // append these "bar"s onto the original String
                (x.0).push_str(&app_str);
            }
        };
    });
    // print the Strings after alteration with their associated Command
    vec_in.iter().for_each(|x| println!("{} {:?}", x.0, x.1));
}

fn main() {
    use crate::my_module::transformer;

    // a vector of tuples
    let mut my_vec = vec![
        ("hello".into(), Command::Uppercase),
        (" all roads lead to rome! ".into(), Command::Trim),
        ("foo".into(), Command::Append(1)),
        ("bar".into(), Command::Append(5)),
    ];

    // for each tuple, transform the string (.0) with the command (.1)
    my_transformer(&mut my_vec);

    // assert tests
    assert_eq!(my_vec[0].0, "HELLO");
    assert_eq!(my_vec[1].0, "all roads lead to rome!");
    assert_eq!(my_vec[2].0, "foobar");
    assert_eq!(my_vec[3].0, "barbarbarbarbarbar");

    // calling just to prevent unused function warnings
    transformer(&mut my_vec);
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!

    // pub fn transformer(input: ???) -> ??? {

    pub fn transformer(vec_in: &mut Vec<(String, Command)>) -> Vec<String> {
        // going to mutate the input vector (in place)
        vec_in.iter_mut().for_each(|x: &mut (String, Command)| {
            // grab the String and Command for this particular tuple element
            let mut _s: String = (x.0).clone();
            let _c: Command = (x.1).clone();
            // perform the Command upon the String
            match _c {
                Command::Uppercase => x.0 = String::from((x.0).as_str().to_uppercase()),
                Command::Trim => x.0 = String::from((x.0).as_str().trim()),
                // grab the Command::Appends variant value
                Command::Append(variant_value) => {
                    let mut app_str: String = String::from("");
                    // append variant_value "bar"s together
                    for _ in 0..variant_value {
                        app_str.push_str("bar");
                    }
                    // append these "bar"s onto the original String
                    (x.0).push_str(&app_str);
                }
            };
        });

        let mut vec_out: Vec<String> = vec![]; 
        for i in 0..vec_in.len() {
            let _s: String = (vec_in[i].0).to_string();
            vec_out.push(_s);
        }
        vec_out
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::Command;
    use crate::my_module::transformer;

    #[test]
    fn it_works() {
        let mut vec_in = vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ];
        let output = transformer(&mut vec_in);
    
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
