use polyester::page::Page;
use polyester::time;
use sevenguis_lib::home_page;
use sevenguis_lib::temperature_page;
use sevenguis_lib::timer_page;
use std::env;
use std::time::SystemTime;

fn main() {
    let args_: Vec<String> = env::args().collect();
    let args: Vec<&str> = args_.iter().map(|s| s.as_ref()).collect();
    let now = posix_now();

    match args[1..] {
        ["home_page"] => {
            let page = home_page::HomePage {};
            render_html(page);
        }

        ["temperature_page"] => {
            let page = temperature_page::TemperaturePage {};
            render_html(page);
        }

        ["timer_page"] => {
            let page = timer_page::TimerPage { initial_time: now };
            render_html(page);
        }

        _ => {
            println!("Invalid command");
        }
    }
}

fn posix_now() -> time::Posix {
    let now = SystemTime::now();
    let millis = now.elapsed().unwrap().as_millis();
    time::Posix::from_millis(millis as i128)
}

fn render_html<Model, Msg, CustomMsg>(page: impl Page<Model, Msg, CustomMsg>) {
    let (model, _effects) = page.init();
    let page = page.view(&model);
    println!("{}", page.to_markup().into_string());
}
