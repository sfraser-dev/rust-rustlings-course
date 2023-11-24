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
    vec_in.iter_mut().for_each(|x: &mut (String, Command)| {
        let mut _s: String = (x.0).clone();
        let _c: Command = (x.1).clone();
        println!("{} {:?}", _s, _c);
        match _c {
            Command::Uppercase => x.0 = String::from((x.0).as_str().to_uppercase()),
            Command::Trim => x.0 = String::from((x.0).as_str().trim()),
            // grab enum variant's value
            Command::Append(grab_value) => {
                println!("val ======= {}",grab_value);
                let mut app_str: String = String::from("");
                for _ in 0..grab_value {
                    app_str.push_str(&x.0);
                }
                println!(".....................{}",app_str);
            },
        };
    });
    //     if _c == Command::Uppercase {
    //         _s.make_ascii_uppercase();
    //     } else {
    //         println!("error: match command");
    //         println!("{:?}",_c)
    //     }
    // });
    for i in 0..vec_in.len() {
        println!("{} {:?}", vec_in[i].0, vec_in[i].1);
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
    // assert_eq!(my_vec[0].0, "HELLO");
    // assert_eq!(my_vec[1].0, "all roads lead to rome!");
    // assert_eq!(my_vec[2].0, "foobar");
    // assert_eq!(my_vec[3].0, "barbarbarbarbarbar");
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
