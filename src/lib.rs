
use std::fs;
use std::error::Error;

#[derive(Debug)]
pub struct Config{
    pub  query : String,
    pub filename : String
}

impl Config{
pub fn new(data : &[String]) -> Result<Self , &str>{
    if data.len() < 3 {
        return Err("3 arguments required");
    }
    let query = data[1].clone();
    let filename = data[2].clone();

    Ok(Self{query , filename})
}
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
  let content = fs::read_to_string(config.filename)?;
  for line in search(&config.query, &content){
    println!("{line}");
  }
//    println!("\n{}",content);
   Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // vec![]
    let mut result = Vec::new();
    for lines in contents.lines(){
        if lines.contains(query){
            result.push(lines);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}