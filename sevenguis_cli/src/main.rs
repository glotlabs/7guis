use sevenguis_lib::home_page;
use polyester::page::Page;
use std::env;

fn main() {
    let args_: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_.iter().map(|s| s.as_ref()).collect();
    let page = home_page::HomePage {};

    match args[1..] {
        ["home_page"] => {
            let (model, _effects) = page.init();
            let page = page.view(&model);
            println!("{}", page.to_markup().into_string());
        }

        _ => {
            println!("Invalid command");
        }
    }
}
