use git::Repository;
use std::io::{Read,self};




pub fn clone_repo(repo:String,path:&str) -> Result<(),String> {

    match Repository::clone(&repo,path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{:}",e))
    }
}


pub fn get_repo() -> Result<String,String> {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf);

    let mut segments = buf.trim().split("/");


    let buf = format!("https://github.com/{}/{}.git",segments.next().unwrap(),segments.next().unwrap());


    Ok(buf)
}