
// mod print;
// mod vars;
// mod types;
 //mod strings;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
 mod functions;
// mod pointer_ref;
//mod structs;
//mod enums;
//mod cli;

fn main() {
    // mut возможно изменить значение 
    //let mut first: bool = true;
    //first = false;
    
  //  let int_data: i8 = 10; //-128 : +127

  //  let number = 10;

   // println!("MAX i8 is {}", std::i8::MAX);
   // println!("MIN i8 is {}", std::i8::MIN);

   // println!("MAX i128 is {}", std::i128::MAX);
   // println!("MIN i128 is {}", std::i128::MIN);


    let example_str: &str = "test1";
    let example_string: String = String::from("Partner");

    let _string_from_str: String = example_str.to_string();
    let _string_from_str2: String = "test hard strig".to_string();

    let _string_from_hard = String::from("some hard");
    let _string_from_str_var = String::from(example_str);

    let _str_from_string: &str = &example_str;

    let _combine_string = ["1", "2"].concat();
    let _combine_string_macro = format!("{} {}", "1", "2");

    let _string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("some hurd");
    //mut_string.push("m");

    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    // tuples::run();
    // arrays::run();
    // vectors::run();
    // conditionals::run();
    // loops::run();
     functions::run();
    // pointer_ref::run();
    //structs::run();
    //enums::run();
    //cli::run();
}
