extern crate lib_clone;

use lib_clone::clone;

fn main() {
    clone::clone_repo(clone::get_repo().expect("Invalid repo name"),"./test").expect("Couldnt clone repo")
}
