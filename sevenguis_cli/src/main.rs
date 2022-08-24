use polyester::page::Page;
use polyester::time;
use sevenguis_core::counter_page;
use sevenguis_core::crud_page;
use sevenguis_core::flight_page;
use sevenguis_core::home_page;
use sevenguis_core::temperature_page;
use sevenguis_core::timer_page;
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

        ["counter_page"] => {
            let page = counter_page::CounterPage {};
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

        ["crud_page"] => {
            let page = crud_page::CrudPage {};
            render_html(page);
        }

        ["flight_page"] => {
            let page = flight_page::FlightPage { initial_time: now };
            render_html(page);
        }

        _ => {
            println!("Invalid command");
        }
    }
}

fn posix_now() -> time::Posix {
    let millis = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    time::Posix::from_millis(millis as i128)
}

fn render_html<Model, Msg, AppEffect>(page: impl Page<Model, Msg, AppEffect>) {
    let (model, _effects) = page.init();
    let page = page.view(&model);
    println!("{}", page.to_markup().into_string());
}
