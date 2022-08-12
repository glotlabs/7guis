use polyester::page::Page;
use sevenguis_lib::home_page;
use sevenguis_lib::temperature_page;
use std::env;

fn main() {
    let args_: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_.iter().map(|s| s.as_ref()).collect();

    match args[1..] {
        ["home_page"] => {
            let page = home_page::HomePage {};
            render_html(page);
        }

        ["temperature_page"] => {
            let page = temperature_page::TemperaturePage {};
            render_html(page);
        }

        _ => {
            println!("Invalid command");
        }
    }
}

fn render_html<Model, Msg, CustomMsg>(page: impl Page<Model, Msg, CustomMsg>) {
    let (model, _effects) = page.init();
    let page = page.view(&model);
    println!("{}", page.to_markup().into_string());
}
