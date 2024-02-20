extern crate enigo;

use enigo::*;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("No arguments provided."),
        2 => match args[1].as_str() {
            "--test" => test(),
            "--gatling_sentry" => gatling_sentry(),
            "--orbital_airburst_strike" => orbital_airburst_strike(),
            "--resupply" => resupply(),
            "--reinforce" => reinforce(),
            "--eagle_cluster_strike" => eagle_cluster_strike(),
            _ => println!("Unknown argument: {}", args[1]),
        },
        _ => println!("Usage: my_program [--help | --version]"),
    }
}

fn test() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Layout('t'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('t'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('e'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('e'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(25));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('t'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('t'));
    sleep(Duration::from_millis(80));
}

fn gatling_sentry() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Control);
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('w'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('w'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(25));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('a'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('a'));
    sleep(Duration::from_millis(80));

    enigo.key_up(Key::Control);
}

fn orbital_airburst_strike() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Control);
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_up(Key::Control);
}

fn resupply() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Control);
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('w'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('w'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_up(Key::Control);
}

fn reinforce() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Control);
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('w'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('w'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('a'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('a'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('w'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('w'));
    sleep(Duration::from_millis(80));

    enigo.key_up(Key::Control);
}

fn eagle_cluster_strike() {
    let mut enigo = Enigo::new();

    enigo.key_down(Key::Control);
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('w'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('w'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('s'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('s'));
    sleep(Duration::from_millis(80));

    enigo.key_down(Key::Layout('d'));
    sleep(Duration::from_millis(30));
    enigo.key_up(Key::Layout('d'));
    sleep(Duration::from_millis(80));

    enigo.key_up(Key::Control);
}
