// This uses crate syntax! Bcuz it runs library from local.
// If you want to use rascii_art (crates.io) library, use rascii_art instead!

use rascii::{
    command_line::{print_and_exit, read_env_args},
    prelude::*,
    search::{index_any_sub_command, search_any_sub_command},
};
use std::process;

// This is driver code to use rascii library:
fn main() {
    // Logo: Created by rascii :D
    let logo = "
   'l_?]]?<:     .;~?]]]]-<;.                                 ':i+-?]]]]~   ![~   ']]\"
  :(/-!;I>[f?.  I(/[>l;;l>[/);                              ^-(0->lI;;;;\"   ]c(.  \"rul
 ]n>     `|xI  ,tfl        !j/\"                            ^(j<'            ]c(.  \"rul
'/n!     `|xI  +u?          [u<     '           `\"\"^      |~u[              ]c(.  \"rul
'/z/~;:;i[f?.  [u>          ~u?   `_)>       ,~[[}}0)~o    |-v~              ]c(.  \"rul
'/u]][)xf-:    [ui          <u]  `)j<.     :]/[!^  \")xi   %~u[              ]c(.  \"rul
'/r:  ^]f>     [v[i!!!!!!!!i]v[  ln('    ,?/0!.    '[n>    ^(j<'            ]c(.  \"rul
'/r,   '[/i    [c(?--------?|c]  ^(f+IIi]|0i.     ^?t?'     ^-(1->lI;;;;\"   ]c(.  \"rul
 _]`    '+_'   >[:          ;]i   `i?]]-<:        ,+;        `:i+-?]]]]~    ![~   ']]\"

 KoBruhh/RASCII created with rust programming language";

    // Error messages returned by programs error ouput:
    /* main errors */
    let path_error =
        "conside including a file path to allow program to find an image to create art!";
    let with_color_error = "you have to enter 3 u8 values to get a RGB value!";
    /* sub errors */
    let parse_error = "you have to use three u8 values to use -wc || --with-color command!";

    // these are are sub_command lists
    let help_codes = vec!["-h", "--help"];
    let path_codes = vec!["-p", "--path"];
    let list_codes = vec!["-l", "--list"];
    let invert_codes = vec!["-i", "--invert"];
    let ratio_codes = vec!["-r", "--ratio"];
    let color_codes = vec!["-c", "--color", "--colored"];
    let pixelated_codes = vec!["-px", "--pixelated"];
    let background_codes = vec!["-bg", "--background"];
    let custom_list_codes = vec!["-l", "--list"];
    let with_color_codes = vec!["-wc", "--with-color"];

    let sub_commands = read_env_args();
    if sub_commands.len() < 2 {
        print_and_exit(logo);
    }

    // Map of Dominant to ressessif commands: (this allows algorithm to be more efficient) -> if map is right, no unnecessary calculations will made.
    // -h, --help -> help_message
    // -p, --path -> if does not exist, error!
    // -r, --ratio -> image has to resized whenever needed! (Can't use it after ASCII translation)
    // -px, --pixelated -> pixelart! // it has to be colored! and cannot override with --with-color command cuz it is nonsense.
    // -l, --list -> has to be before ASCII translation, cuz translation is dependent on char_list.
    // -i, --invert -> inverting the black and white for ASCII output.
    // -wc, --with-color -> initalizer for text color
    //                  | --colored ,if there is no initalizer for bg, and -c is included creates a colored ouput.
    // -bg, --background -> creates pixelart on the background, ASCII characters are still displayed.

    if search_any_sub_command(&help_codes, &sub_commands) {
        print_help(); // prints help info on bottom!
    } else {
        // If you type -h || --help only help is going to be displayed!
        let path = dbg!(&sub_commands[index_any_sub_command(&path_codes, &sub_commands).unwrap_or_else(|i| {
                                    print_and_exit(path_error);
                                }) + 1]);
        let ratio = dbg!(if let Ok(index) = index_any_sub_command(&ratio_codes, &sub_commands) {sub_commands[index + 1].parse().unwrap_or_else(|_| {
                    print_and_exit(parse_error); // <- could change this error type
                })} else {100});
        let list = dbg!(if let Ok(index) = index_any_sub_command(&list_codes, &sub_commands) {&sub_commands[index + 1]} else {"default"});
        let invert = search_any_sub_command(&invert_codes, &sub_commands);
        if search_any_sub_command(&color_codes, &sub_commands) {
            unimplemented!(); // has to create colored image!
        } else {
            // if no decleration for colored input, else block is gonna run: has to create one colored image;
            let text_color: Vec<u8> =
                if let Ok(i) = index_any_sub_command(&with_color_codes, &sub_commands) {
                    if sub_commands[i..].len() < 4 {
                        // elimination of panic! macro on overindexing!
                        print_and_exit(with_color_error);
                    } else {
                        let rgb = &sub_commands[i + 1_usize..i + 4_usize];
                        rgb.iter()
                            .map(|v| {
                                v.parse::<u8>().unwrap_or_else(|_| {
                                    print_and_exit(parse_error);
                                })
                            })
                            .collect()
                    }
                } else {
                    // convert_to_ascii(); // not colored!
                    println!("{}", convert_to_colored_ascii(path, [255, 255, 255], list, ratio, invert));
                    Vec::new()
                };
            // create ASCII with given color and path!
        }
    }
}

fn print_help() {
    println!("help for rascii tool!");
}
