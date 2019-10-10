extern crate iron;
extern crate params;

extern crate reqwest;


mod script_template;
//use std::env;

//use std::fs;

use script_template::get_script_template;
use iron::prelude::*;
//use iron::status;
use params::{Params, Value};

fn hello_world(req: &mut Request) -> IronResult<Response> {
    
    let map = req.get_ref::<Params>().unwrap();

    match map.find(&["script_template"]) {
        Some(&Value::String(ref tmpl)) => {
            let the_template = get_script_template(tmpl);
            Ok(Response::with((iron::status::Ok, the_template)))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }


    //let file_name = "./the_file.txt";
    //    let contents = fs::read_to_string(file_name)
    //    .expect("Something went wrong reading the file");

    //let contents = DATA_SET;

    //Ok(Response::with((status::Ok, contents)))
}
//const DATA_SET:&'static str = include_str!("../the_file.txt");


fn main() -> Result<(), Box<dyn std::error::Error>> {

    println!("GET https://www.rust-lang.org");

    let mut res = reqwest::get("https://www.rust-lang.org/")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:?}", res.headers());

    // copy the response body directly to stdout
    std::io::copy(&mut res, &mut std::io::stdout())?;


    let chain = Chain::new(hello_world);
    let r = Iron::new(chain).http("0.0.0.0:3000");
    match r {
        Ok(_res) => {
            // all ok
        }
        Err(e) => {
            println!("Another instance running on port 3000??");
            println!("{}",e);
        }
    }
    Ok(())
}
