use std::{error::Error, fs};
pub struct Config{
    query:String,
    filename:String
}

impl Config {
    pub fn new(args:Vec<String>)->Result<Self,&'static str>{
        if args.len()<3{
            return Err("no enough argument!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query,filename})
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let result = search(&config.query, &contents);
    for line in result{
        println!("{}",line);
    }
    Ok(())
}

pub fn search<'a>(query:&str, contents:&'a str)->Vec<&'a str>{
    let mut res = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests{

    use super::*;

    fn one_result(){
        let query = "to";
        let contents = "\
        I'm nobody! Who are you?
        Are you nobody, too?
        Then there's a pair of us - don't tell!
        They'd banish us, you know.
        
        How dreary to be somebody!
        How public, like a frog
        To tell your name the livelong day
        To an admiring bog!";
        assert_eq!(vec!["Are you nobody, too?","How dreary to be somebody!"],search(&query,&contents));
    }
}