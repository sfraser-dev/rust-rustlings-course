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

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

fn my_transformer(vec_in: &mut Vec<(String, Command)>) {
    let the_vec_iter = vec_in.iter();
    for tuple_element in the_vec_iter {
        println!("{}",tuple_element.0);
    }
}

fn main() {
    // a vector of tuples
    let mut my_vec = vec![
        ("hello".into(), Command::Uppercase),
        (" all roads lead to rome! ".into(), Command::Trim),
        ("foo".into(), Command::Append(1)),
        ("bar".into(), Command::Append(5)),
    ];

    // for each tuple, transform the string (.0) with the command (.1)
    my_transformer(&mut my_vec);

    // tests
    assert_eq!(my_vec[0].0, "HELLO");
    assert_eq!(my_vec[1].0, "all roads lead to rome!");
    assert_eq!(my_vec[2].0, "foobar");
    assert_eq!(my_vec[3].0, "barbarbarbarbarbar");

    println!("{}",my_vec[0].0)
}

// mod my_module {
//     use super::Command;

//     // TODO: Complete the function signature!
//     pub fn transformer(input: ???) -> ??? {
//         // TODO: Complete the output declaration!
//         let mut output: ??? = vec![];
//         for (string, command) in input.iter() {
//             // TODO: Complete the function body. You can do it!
//         }
//         output
//     }
// }

// #[cfg(test)]
// mod tests {
//     // TODO: What do we need to import to have `transformer` in scope?
//     use my_module::transformer;
//     use super::Command;

//     #[test]
//     fn it_works() {
//         let output = transformer(vec![
//             ("hello".into(), Command::Uppercase),
//             (" all roads lead to rome! ".into(), Command::Trim),
//             ("foo".into(), Command::Append(1)),
//             ("bar".into(), Command::Append(5)),
//         ]);
//         assert_eq!(output[0], "HELLO");
//         assert_eq!(output[1], "all roads lead to rome!");
//         assert_eq!(output[2], "foobar");
//         assert_eq!(output[3], "barbarbarbarbarbar");
//     }
// }
