use std::env;
use std::process;

// Import the lib that has all of the functionality 
// use mini_grep::Config; // We have imported using mini_grep which is the name of project , instead of lib::config
// Rust is smart enough to know this where the config is the project and its the way most people import lib files
use mini_grep::Config;
fn main() {
  let arg : Vec<String> = env::args().collect();
  let config = Config::new(&arg).unwrap_or_else(|error|{
    // println!("ERROR..🟥🟥🟥🟥🟥: {} ",error );
    process::exit(1);
  });
 if let Err(e) =mini_grep::run(config){
    // println!("Error was found : {}" , e);
    process::exit(1);
 }
}

// fn run(config:Config) -> Result<(),Box<dyn Error>> {
//   let content = fs::read_to_string(config.filename)?;
// //    println!("\n{}",content);///
//    Ok(())
// }


// #[derive(Debug)]
// struct Config{
//     query : String,
//     filename : String
// }

// impl Config{
// fn new(data : &[String]) -> Result<Self , &str>{
//     if data.len() < 3 {
//         return Err("3 arguments required");
//     }
//     let query = data[1].clone();
//     let filename = data[2].clone();

//     Ok(Self{query , filename})
// }
// }

// use mini_grep::Config;

// fn main(){
//     if let Err(e) = mini_grep::run(config){

//     }
// }