mod macro_derive_traits;
use macro_derive_traits::CheckStringFields;
extern crate macro_derive;
use macro_derive::CheckStringFields;


// #[derive(HelloMacro)]
#[derive(Debug,CheckStringFields)]
pub struct Test{
    pub a: String,
    pub b: String,
    pub c: usize
}

fn main() {
    let t1 = Test{
        a:String::from("foo"),
        b:String::from("bar"),
        c: 10
    };
    let t2 = Test{
        a:String::from("foo"),
        b:String::from(""),
        c: 10
    };
    match t1.check(){
        true => {}
        _ => {
            println!("please check struct fields!");
            println!("{:?}",t1);
        }
    };
    match t2.check(){
        true => {}
        _ => {
            println!("please check struct fields!");
            println!("{:?}",t2);
        }
    };
}
