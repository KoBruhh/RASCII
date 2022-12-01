use ::image::{imageops::FilterType::Gaussian, GenericImageView, ImageError, Rgba, open};

use std::{collections::HashMap, env, process};

type XyA = ((u32, u32), u8);
type XyRgb = ((u32, u32), (u8, u8, u8));
type XyRgba = ((u32, u32), (u8, u8, u8, u8));
type XyIRgb = ((u32, u32), f64, (u8, u8, u8));

pub trait Colored {
    fn take_saturation(&self) -> XyA;
    fn take_color(&self) -> XyRgb;
    fn take_appearance(&self) -> XyRgba;
}

#[derive(Debug)]
pub struct Img {
    body: Vec<Pixel>,
}

#[derive(Debug)]
pub struct Pixel((u32, u32), (u8, u8, u8, u8));

impl Colored for Pixel {
    fn take_saturation(&self) -> XyA {
        unimplemented!()
    }
    fn take_color(&self) -> XyRgb {
        unimplemented!();
    }
    fn take_appearance(&self) -> XyRgba {
        unimplemented!();
    }
}

impl Img {
    fn take_saturation(&self) -> Vec<XyA> {
        let mut result = Vec::new();
        for pixel in self.body.as_slice() {
            let Pixel((x, y), (_, _, _, saturation)) = pixel;
            result.push(((*x, *y), *saturation));
        }
        result
    }
    fn take_color(&self) -> Vec<XyRgb> {
        let mut result = Vec::new();
        for pixel in self.body.as_slice() {
            let Pixel((x, y), (r, g, b, _)) = pixel;
            result.push(((*x, *y), (*r, *g, *b)));
        }
        result
    }
    fn take_appearance(&self) -> Vec<XyRgba> {
        let mut result = Vec::new();
        for pixel in self.body.as_slice() {
            let Pixel((x, y), (r, g, b, saturation)) = pixel;
            result.push(((*x, *y), (*r, *g, *b, *saturation)));
        }
        result
    }
}

impl Img {
    pub fn new() -> Self {
        Img { body: Vec::new() }
    }
    pub fn from(pixels: Vec<Pixel>) -> Self {
        Img { body: pixels }
    }
}

pub mod command_line{
    use std::env::args;
    use super::process;
    pub fn read_env_args() -> Vec<String> {
        args().into_iter().collect() // Idk if it's the best way to do it.
    }

    pub fn print_and_exit(err_msg: &str) -> ! {
        eprintln!("{}", err_msg);
        process::exit(1)
    }
}

// returns found sub_commands from given sub_command list!
pub mod search {
    use std::fmt;

    #[derive(Debug)]
    pub struct SearchError(String); // custom error type that returns an error message.
    
    impl fmt::Display for SearchError {
        fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub fn search_exact_sub_command(search_list: &Vec<&str>, sub_command_list:&Vec<String>) -> bool {
        let mut bool_list = Vec::with_capacity(search_list.len());
        for command in search_list {
            bool_list.push(sub_command_list.contains(&command.to_string()));
        }
        !bool_list.iter().any(|i| *i == false) // capturing false value if exists!
    }

    pub fn search_any_sub_command(search_list: &Vec<&str>, sub_command_list:&Vec<String>) -> bool {
        let mut bool_list = Vec::with_capacity(search_list.len());
        for command in search_list {
            bool_list.push(sub_command_list.contains(&command.to_string()));
        }
        bool_list.iter().any(|i| *i == true) // capturing true value if exists!
    }
    
    pub fn index_exact_sub_command(search_list: &Vec<&str>, sub_command_list:&Vec<String>) -> Result<Vec<(String, usize)>, SearchError> { // returns all subcommands and their indexes, If exists! Else it panics.
        if !search_exact_sub_command(search_list, sub_command_list) { // if not all elements pair:
            Err(SearchError(String::from("Keywords and search list does not match! You can only find indexes from existing keywords!")))
        } // if you want to find an index! first you have to include the variable! This function has to have all of the variables used in search!
        else {
            let mut sub_command_index_list:Vec<(String, usize)> = Vec::with_capacity(search_list.len());
            search_list.iter().for_each(|key| {
                sub_command_index_list.push((key.to_string(), sub_command_list.iter().position(|x| x == key).expect("Couldn't pair keyword! Lists doesn't match!") )); // it has to panic if lists does not pair!
            });
            Ok(sub_command_index_list)
        }
    }

    pub fn index_any_sub_command(search_list: &Vec<&str>, sub_command_list:&Vec<String>) -> Result<usize, SearchError> { // returns the index of first satisfied subcommand, If exists!
        if !search_any_sub_command(search_list, sub_command_list) { // If no pair:
            Err(SearchError(String::from("Keywords and search list does not match! You can only find indexes from existing keywords!")))
        } // if you want to find an index! first you have to include the variable! This function has to have all of the variables used in search!
        else {
            for key in search_list {
                if sub_command_list.iter().any(|i| i == key) {
                    return Ok(sub_command_list.iter().position(|x| x == key).expect("Couldn't pair keyword! Lists doesn't match!")); // have to fix these panics later!
                }
            };
           Err(SearchError(String::from("Couldn't find a pair"))) // Default value will be mutated if any pairs gets detected.
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_search_any_sub_command() {
            let search_list_true = vec!["-h", "--help", "--path"];
            let search_list_false = vec!["--no-help-here", "--help", "--path"];
            let sub_command_list= vec![String::from("-h")];

            assert!(search_any_sub_command(&search_list_true, &sub_command_list) == true);
            assert!(search_any_sub_command(&search_list_false, &sub_command_list) == false);
        }
        
        #[test]
        fn test_search_exact_sub_command() {
            let search_list_true = vec!["-h", "--path"];
            let search_list_false = vec!["-h", "--help", "--path-is-absend"];
            let sub_command_list= vec![String::from("-h"), String::from("--path"), String::from("--another-subcommand")]; // it doesn't matter how much do you have in subcommand_list, We just want to know that all of searchs have a match 

            assert!(search_exact_sub_command(&search_list_true, &sub_command_list) == true);
            assert!(search_exact_sub_command(&search_list_false, &sub_command_list) == false);
        }

        #[test]
        fn test_index_any_sub_command() {
            let search_list_true = vec!["-h", "--path", "-no-sense"];
            let search_list_true_2 = vec!["--not_first", "--path", "-no-sense"];
            let search_list_false = vec!["--help", "--path-is-absend"];
            let sub_command_list= vec![String::from("-h"), String::from("--path"), String::from("--another-subcommand")]; // it doesn't matter how much do you have in subcommand_list, We just want to know that all of searchs have a match 

            assert!(dbg!(index_any_sub_command(&search_list_true, &sub_command_list).unwrap()) == (String::from("-h"), 0_usize));
            assert!(dbg!(index_any_sub_command(&search_list_true_2, &sub_command_list).unwrap()) == (String::from("--path"), 1_usize));
        }
        #[test]
        fn test_index_exact_sub_command() {
            let search_list_true = vec!["-h", "--path"];
            let search_list_true_2 = vec!["-h"];
            let sub_command_list= vec![String::from("-h"), String::from("--path")]; // it doesn't matter how much do you have in subcommand_list, We just want to know that all of searchs have a match 

            assert!(dbg!(index_exact_sub_command(&search_list_true, &sub_command_list).unwrap()) == vec![(String::from("-h"), 0_usize), (String::from("--path"), 1_usize)]);
            assert!(dbg!(index_exact_sub_command(&search_list_true_2, &sub_command_list).unwrap() == vec![(String::from("-h"), 0_usize)]));
            // false option is going to be tested after I clear expect(...) -> It panics now!
        }
    }
}

mod image {
    use super::*;
    fn open_img(path:&str, ratio:usize) -> Result<Img, ImageError> {
        let mut img = image::open(path)?;
        // if !input_is_cubic_char {
        //     img = img.resize_exact(img.width(), img.height() / 2, Gaussian);
        // }

        let mut result: Img = Img::new();
        let pixels: Vec<(u32, u32, Rgba(u8, u8, u8, u8))> = img.pixels().into_iter().collect();

        Ok::<Img, ImageError>(result)
}
}

pub mod prelude {
    use super::*;
    const builtin_char_map:HashMap<&str,&str> = HashMap::from([ // List of available char_lists to be used on conversion:
        ("default", "..."),
        ("japanese", "ッツヅミテデトドナぁあぃいぅうぇえぉおかがきぎけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ゛゜ゝゞゟ゠ァアィイカガキギクグケゲコゴサザシジスズセゼソゾタダチヂニヌネノハバパヒビピフブプヘベペホボポマムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶヷヸヹヺヾくぐゥウェエォオ、。"),
        ("chinese", "厅和酒店等私空间安装面部识别摄像头在个案例中调查发现东南部福建省福州市的公安希望在美国酒店品牌戴斯酒店特许经营店的大堂内安装摄像头酒店前台经理告诉纽约时报摄像头没有脸识别功能也没有和公安网络联网文件显示福州市公安局还要求接家当地喜来登酒店内的视频监控资，。，。"),
        ("russian", "АБВГДЕЁЖЗИЙКЛМНОПPСТУФХЦЧШЩЪЫЬЭЮЯ  "),
        //         // ("hindi", "कखगघङचछजझञटठडढणतथदधनपफबभमयरलवशषसहक्त्ज्ञ"), //this does not work bcuz hindi
        //         // chars are dependent to each other, so there is no linear ratio!
        //         // all lines has to contain same amount of pixels!
        //         // but hindi chars have dependent width!
        //         // thats why I am not doing it!
        ("emoji", "😆😅🤣😉😊😋🤩😝🤑🤐🤗😛🧐😶😐🙄😠🤬😡😔🙁😫😩😮😱😨😰😯😓😭🤕🤢🤮🥵🥶🥳🥸😴💤🤡👹🦀👺💀👻👽👾😺🙀😿😾🙌🤛🤚👊🤞🤘👌🤏💪🦾🦿🤝🙏👆🖖💅🤳👄🦀👿🦀👅👂🦻👃🧠🫀🫁🦷🦴👁👀👤👥🗣👶🧔，。"),
        ("ansi", "█▓▒░ "),
        ("slight", "$@%8W*adpLY\\|){[?_~>!I:\"`.   "),
    ]); // contains supported char_lists
    pub fn convert_with_color(path:&str, color:[u8;3], list:&str /*char_list_name*/ , ratio:usize) -> String {
        let list = if let Some(list) = builtin_char_map.get(list) 
        {list} else {list}; // so much lists!
        let is_cubic_char = if list == "japanese" || list == "chinese" || list == "emoji" { // these are cubic characters
            true
        } else {false};
        ;
    }
    // pub fn convert() -> String {
    //                 let pixelated = sub_commands
    //         .iter()
    //         .any(|i| i == "-px" || i == "--pixelated");
    //     let with_color = sub_commands
    //     .iter()
    //     .any(|i| i == "-wc" || i == "--with_color");
    //     let full_background = sub_commands
    //     .iter()
    //     .any(|i| i == "-bg" || i == "--background");
    //     let reverse = sub_commands.iter().any(|i| i == "-i" || i == "--invert");
    //     let colored = sub_commands
    //     .iter()
    //     .any(|i| i == "-c" || i == "--colored" || i == "--color");
    //     let supported_char_lists = HashMap::from([ // List of available char_lists to be used on conversion:
    //         ("japanese", "ッツヅミテデトドナぁあぃいぅうぇえぉおかがきぎけげこごさざしじすずせぜそぞただちぢっつづてでとどなにぬねのはばぱひびぴふぶへべぺほぼぽまみむめもゃやゅゆょよらりるれろゎわゐゑをんゔゕゖ゛゜ゝゞゟ゠ァアィイカガキギクグケゲコゴサザシジスズセゼソゾタダチヂニヌネノハバパヒビピフブプヘベペホボポマムメモャヤュユョヨラリルレロヮワヰヱヲンヴヵヶヷヸヹヺヾくぐゥウェエォオ、。"),
    //         ("chinese", "厅和酒店等私空间安装面部识别摄像头在个案例中调查发现东南部福建省福州市的公安希望在美国酒店品牌戴斯酒店特许经营店的大堂内安装摄像头酒店前台经理告诉纽约时报摄像头没有脸识别功能也没有和公安网络联网文件显示福州市公安局还要求接家当地喜来登酒店内的视频监控资，。，。"),
    //         ("russian", "АБВГДЕЁЖЗИЙКЛМНОПPСТУФХЦЧШЩЪЫЬЭЮЯ  "),
    //         // ("hindi", "कखगघङचछजझञटठडढणतथदधनपफबभमयरलवशषसहक्त्ज्ञ"), //this does not work bcuz hindi
    //         // chars are dependent to each other, so there is no linear ratio!
    //         // all lines has to contain same amount of pixels!
    //         // but hindi chars have dependent width!
    //         // thats why I am not doing it!
    //         ("emoji", "😆😅🤣😉😊😋🤩😝🤑🤐🤗😛🧐😶😐🙄😠🤬😡😔🙁😫😩😮😱😨😰😯😓😭🤕🤢🤮🥵🥶🥳🥸😴💤🤡👹🦀👺💀👻👽👾😺🙀😿😾🙌🤛🤚👊🤞🤘👌🤏💪🦾🦿🤝🙏👆🖖💅🤳👄🦀👿🦀👅👂🦻👃🧠🫀🫁🦷🦴👁👀👤👥🗣👶🧔，。"),
    //         ("ansi", "█▓▒░ "),
    //         ("slight", "$@%8W*adpLY\\|){[?_~>!I:\"`.   "),
    //     ]); // contains supported char_lists
    // let print_help = || {
    // println!(
    //     "   ~This program allows you to create ASCII art from any image (.png, .jpeg, .jpg ...)~
    // SubCommands (rascii <SubCommand>):
    //     -p, --path <path/to/image> // given path is the image path that you want to convert.
    //     -i, --invert // to be able to create images without white background. (if your image has a white plain and you want to ignore it use this command)
    //     -c, --colored // to make ASCII art colored
    //     -h, --help // to access this window
    //     -r, --ratio // to resize img, It could be higher than hundred! but can't be lower than zero!
    //     -bg, --background // to paint background with the color of the pixel!
    //     -wc, --with-color <R> <G> <B> // to create custom colored ASCII arts! you could mix these with -bg !
    //     -px, --pixelated // to convert output to pixelart
    //     -l <your_char_list_or `Available lists at bottom appendix I`> --list <your_char_list_or `Appendix I`> // allows you to create custom lists! and use builtin char_lists!

    // |If you dont understand, Just ask your mom|
    // ||This project is rusty, so it is fast! really!||

    // Appendix I
    //     Don't forget that you could enter your own list by:
    //         rascii -p <path/to/path> -l <your_char_list_or || names below!>
    //     Exmpl:
    //         rascii -p <path/to/path> -l bruh // creats your image by using ['b', 'r', 'u', 'h']
    //         rascii -p <path/to/path> -l emoji // creates your image by using emojis -> bcuz list below contains a list named: \"emoji\"!
    //     Builtin_lists: {:#?}
    // ", supported_char_lists.keys());
    // process::exit(1);
    // }; // printing help to std::out
    // if sub_commands.iter().any(|i| i == "-h" || i == "--help") {
    // print_help();
// }

    // let path = if sub_commands
// .iter()
    // .any(|i| (i == "-p" || i == "--path") && (i != "-h" || i != "--help"))
    // {
    //     &sub_commands[&sub_commands
    //         .iter()
    //         .position(|i| i == "-p" || i == "--path")
    //         .unwrap_or_else(|| {
    //             eprint!(
    //                 "
    //         You have to enter:
    //         rascii -p <path>
    //         /* or */
    //         rascii --path <path>
    //         -> to be able to convert an image from given path!
    //         -> Program has to know where is the image you are looking!"
    //             );
    //             process::exit(1);
    //         })
    //     + 1]
    // } else {
    //     eprintln!(
    //         "Help: Consider adding:
    // rascii --path <path>
    // rascii -p <path>
    //     to your command.
    // more help:
    // rascii --help or -h"
    //     );
    //     process::exit(1);
    // };
    // let ratio: usize = if sub_commands.iter().any(|i| i == "-r" || i == "--ratio") {
    // sub_commands[sub_commands
    //     .iter()
    //     .position(|i| i == "-r" || i == "--ratio")
    //     .expect("This cannot raise an error please report this via github issues on KoBruhh/RASCII") // This cannot raise any error!
    //     + 1]
    //     .parse()
    //     .unwrap_or_else(|_| {
    //         eprintln!(
    //             "You cannot enter value below zero!
    // Exmpl:
    //     rascii -p <path/to/img> -r -1 -> unvalid! because program cannot calculate a negative ratio!
    //     rascii -p <path/to/img> -r 1000 -> valid! even ratio is higher than %100 you are able to boost it by using higher values of ratio!
    //     rascii -p <path/to/img> -r 50 -> valid! its between %100 - 0 no big deal. But check the upper command! ^^^^^^^^"
    //         );
    //         process::exit(1);
    //     })
    // } else {
    // 100 // If there is no ratio declaration, take it as hundred percent!
    // };
    // let text_color: Vec<u8> = if with_color {
    // let i = sub_commands
    //     .iter()
    //     .position(|i| i == "-wc" || i == "--with-color")
    //     .expect(
    //         "This cannot raise an error please report this via github issues on KoBruhh/RASCII",
    //     );
    // if sub_commands[i..].len() < 3 || sub_commands[i..i + 3].contains(&String::from("-")) {
    //     eprintln!(
    //         "when you use --with-color || -wc
    //     You have to enter 3 unsigned integers (0 - 255) !
    //     Exmpl:
    //         rascii -p <path/to/img> -bg -wc -1 0 123 -> unvalid! has to positive (unsigned) |-1 is negative!|
    //         rascii -p <path/to/img> -bg -wc 244 -> unvalid! you have to satisfy all 3 elements containing (R, G, B) |244 is  only one of them!|
    //         rascii -p <path/to/img> -bg -wc 141 35 0 -> valid!");
    //     process::exit(1);
    // }
    // sub_commands[i + 1..i + 4].iter().map(|i| -> u8 {i.parse().unwrap_or_else(|_| {
    //     eprintln!("could not parse command! when you enter -wc or --with-color following 3 elements has to bu 8bit unsigned integers (0 - 255)");
    //     process::exit(1)
    // })
    // }).collect::<Vec<u8>>() // return value :D
    // } else {
    // vec![0, 0, 0] // If you dont specify a text color by default its going to be white.
    // };

    // let mut input_is_cubic_char = false; // This allows me to detect cubic chars when a language such as japanese is used. Bcuz japanese chars are same in width and height, I dont need to refactor image as I do with latin chars! (latin chars are rectangular!)

    // // Multiple white_spaces to be able to capture slight grayish white colors!
    // let char_list = if sub_commands.iter().any(|i| i == "-l" || i == "--list") {
    // let mut list_info: &str = &sub_commands[&sub_commands
    //     .iter()
    //     .position(|i| i == "-l" || i == "--list")
    //     .expect("This cannot raise an error please report this via github issues on KoBruhh/RASCII") // This cannot raise any error!
    //     + 1];
    // for (name, list) in &supported_char_lists {
    //     if &list_info == name {
    //         list_info = list;
    //         input_is_cubic_char = name == &"chinese" || name == &"japanese" || name == &"emoji";
    //     }
    // }
    // list_info // returns list_info bcuz no pair detected! This allows you to enter your own lists stored on terminal environment variables or directly from -l <your_char_list>
// } else {
    // "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'.    "
    //     // currently 70 elements. But you can modify this list and code will adapt to it.
    // };

    // let open_img = |path: &str, ratio: usize| {
    // let mut img = image::open(path)?;
    // if !input_is_cubic_char {
    //     img = img.resize_exact(img.width(), img.height() / 2, Gaussian);
    // }
    // if !pixelated && !full_background {
    //     img = img.resize_exact(
    //         img.width() * ratio as u32 / 100,
    //         img.height() * ratio as u32 / 100,
    //         Gaussian,
    //     );
    // } else if colored {
    //     img = img.resize_exact(
    //         img.width() * ratio as u32 / 200,
    //         img.height() * ratio as u32 / 100,
    //         Gaussian,
    //     );
    // }

    // let pixels: Vec<(u32, u32, Rgba<u8>)> = img.pixels().into_iter().collect();
    // let mut result: Img = Img::new();

    // for i in pixels.iter() {
    //     let (ax, ay, Rgba([x, y, z, v])) = i;
    //     result.body.push(Pixel((*ax, *ay), (*x, *y, *z, *v)));
    // }

    // Ok::<Img, ImageError>(result)
    // };

    // let img = open_img(path, ratio).unwrap_or_else(|_| {
    // eprint!("Couldn't open image file in path!");
    // process::exit(1);
    // });

    // let appearance = img.take_appearance();

    // let mut regulated: XyIRgb = Vec::with_capacity(appearance.len());
    // for i in appearance.iter() {
    // let ((x, y), (r, g, b, a)) = i;
    // let fine = (
    //     (*x, *y),
    //     *r as f64 / 1020_f64
    //     + *g as f64 / 1020_f64
    //     + *b as f64 / 1020_f64
    //     + *a as f64 / 1020_f64,
    //     (*r, *g, *b),
    // ); // 1020 is 255*4 , when i sum them out,  I want to use them as 0-1 decimal numbers.
    // regulated.push(fine);
    // }

    // let mut y_init = 0; // newline counter
    // let regulated: String = regulated
    // .iter()
    // .map(|i| {
    //     let index = if !reverse {
    //         (i.1 * (char_list.chars().count() - 1) as f64) as usize
    //     } else {
    //         (char_list.chars().count() - 1)
    //             - (i.1 * (char_list.chars().count() - 1) as f64) as usize
    //     };
    //     let ascii_char = char_list
    //         .chars()
    //         .nth(index)
    //         .unwrap_or_else(|| {
    //             eprintln!(
    //                 "
    // You entered an unvalid path!
    //     If your path has white_spaces, you have to change the name of the file!
    //     Exmpl:
    //         rascii -p Img (1).jpeg -> unvalid!
    //         rascii -p Img(1).jpeg -> valid!

    //     Also:
    //         you cannot add ½¾$£ etc. characters to your custom list!
    //     Exmpl:
    //         rascii -p <path/to/img> -l $#$½£ -> unvalid!
    //         rascii -p <path/to/img> -l 123qe.:,- -> valid!"
    //             );
    //             process::exit(1);
    //         })
    //     .to_string();
    //     if i.0 .1 > y_init {
    //         /* the y value of pixel */

    //         y_init = i.0 .1;
    //         if !colored && !with_color {
    //             print!("\n{}", ascii_char);
    //             return stringify!("\n{}", ascii_char);
    //         } else {
    //             let (r, g, b) = i.2;
    //             if !pixelated && !full_background && !with_color {
    //                 let result = format!("\n\x1b[38;2;{r};{g};{b}m{}", ascii_char);
    //                 print!("{result}");
    //                 return stringify!(result);
    //             }
    //             if pixelated {
    //                 let result = format!(
    //                     "\n\x1b[48;2;{r};{g};{b}m \x1b[38;2;{r};{g};{b}m{content}\x1b[0m",
    //                     content = ascii_char
    //                 );
    //                 print!("{result}");
    //                 return stringify!(result);
    //             }
    //             if full_background {
    //                 let result = format!(
    //                     "\n\x1b[48;2;{r};{g};{b}m \x1b[38;2;{};{};{}m{}\x1b[0m",
    //                     &text_color[0], &text_color[1], text_color[2], ascii_char
    //                 );
    //                 print!("{result}");
    //                 return stringify!(result);
    //             }
    //             if with_color {
    //                 let result = format!(
    //                     "\n\x1b[38;2;{};{};{}m{}",
    //                     &text_color[0], &text_color[1], &text_color[2], ascii_char
    //                 );
    //                 print!("{result}");
    //                 return stringify!(result);
    //             }
    //         }
    //     }

    //     if !colored && !with_color {
    //         print!("{ascii_char}");
    //         stringify!(ascii_char)
    //     } else {
    //         let (r, g, b) = i.2;
    //         if !pixelated && !full_background && !with_color {
    //             let result = format!("\x1b[38;2;{r};{g};{b}m{}", ascii_char);
    //             print!("{result}");
    //             return stringify!(result);
    //         }
    //         if pixelated {
    //             let result = format!(
    //                 "\x1b[48;2;{r};{g};{b}m \x1b[38;2;{r};{g};{b}m{content}\x1b[0m",
    //                 content = ascii_char
    //             );
    //             print!("{result}");
    //             return stringify!(result);
    //         }
    //         if full_background {
    //             let result = format!(
    //                 "\x1b[48;2;{r};{g};{b}m \x1b[38;2;{};{};{}m{}\x1b[0m",
    //                 &text_color[0], &text_color[1], text_color[2], ascii_char
    //             );
    //             print!("{result}");
    //             return stringify!(result);
    //         }
    //         if with_color {
    //             let result = format!(
    //                 "\x1b[38;2;{};{};{}m{}",
    //                 &text_color[0], &text_color[1], &text_color[2], ascii_char
    //             );
    //             print!("{result}");
    //             stringify!(result)
    //         } else {
    //             eprintln!("Impossible! report on github issues!");
    //             process::exit(1);
    //         }
    //     }
    // })
    // .collect();

    //     regulated
    //     }

    // pub fn read_env_args() -> Vec<String> {
    // let mut ret = Vec::new();
    // for arg in env::args() {
    //     ret.push(arg);
    // }
    // ret
    // }

}
