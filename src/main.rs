extern crate enigo;

use enigo::*;
use rand::Rng;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You need to input args");
        return;
    }

    let command = match args[1].as_str() {
        "test" => "test",
        "machine_gun_sentry" => "swddw",
        "gatling_sentry" => "swda",
        "mortar_sentry" => "swdds",
        "guard_dog" => "swawds",
        "autocannon_sentry" => "swdwaw",
        "rocket_sentry" => "swdda",
        "ems_mortar_sentry" => "sswwa",
        "anti-personnel_minefield" => "sawd",
        "supply_pack" => "saswws",
        "grenade_launcher" => "sawas",
        "laser_cannon" => "saswa",
        "incendiary_mines" => "saas",
        "guard_dog_rover" => "swawdd",
        "ballistic_shield_backpack" => "sawwd",
        "arc_thrower" => "sdwas",
        "shield_generator_pack" => "swadad",
        "orbital_precision_strike" => "ddw",
        "orbital_gas_strike" => "ddsd",
        "orbital_ems_strike" => "ddas",
        "orbital_smoke_strike" => "ddsw",
        "hmg_emplacement" => "swadda",
        "shield_generation_relay" => "swasdd",
        "tesla_tower" => "swdwad",
        "eagle_strafing_run" => "wdd",
        "eagle_airstrike" => "wdsd",
        "eagle_cluster_bomb" => "wdssd",
        "eagle_napalm_airstrike" => "wdsw",
        "jump_pack" => "swwsw",
        "eagle_smoke_strike" => "wdws",
        "eagle_110mm_rocket_pods" => "wdwa",
        "eagle_500kg_bomb" => "wdsss",
        "orbital_gatling_barrage" => "dsaww",
        "orbital_airburst_strike" => "dd_d",
        "orbital_120mm_he_barrage" => "dsads",
        "orbital_380mm_he_barrage" => "dswwass",
        "orbital_walking_barrage" => "ddsads",
        "orbital_lasers" => "dswds",
        "orbital_railcannon_strike" => "dwssd",
        "machine_gun" => "saswd",
        "anti-material_rifle" => "sadws",
        "stalwart" => "saswwa",
        "expendable_anti-tank" => "ssawd",
        "recoiled_rifle" => "sadda",
        "flamethrower" => "sawsw",
        "autocannon" => "saswwd",
        "railgun" => "sdswad",
        "spear" => "sswss",
        "sos_beacon" => "wsaw",
        "resupply" => "sswd",
        "reinforce" => "wsdaw",
        "hellbomb" => "swaswdsw",
        _ => {
            println!("Unknown argument: {}", args[1]);
            return;
        }
    };

    run_macro(command, args);
}

fn run_macro(command: &str, args: Vec<String>) {
    let mut enigo = Enigo::new();
    let mut rng = rand::thread_rng();

    let lower_down: u64 = 15;
    let upper_down: u64 = 16;

    let lower_up: u64 = 14;
    let upper_up: u64 = 24;

    let ctrl_delay: u64 = rng.gen_range(lower_down..upper_down);

    if args.len() < 3 {
        enigo.key_down(Key::Control);
        sleep(Duration::from_millis(ctrl_delay));

        for c in command.chars() {
            let down_press_delay: u64 = rng.gen_range(lower_down..upper_down);
            let up_press_delay: u64 = rng.gen_range(lower_up..upper_up);
            let key = Key::Layout(c);
            enigo.key_down(key);
            sleep(Duration::from_millis(down_press_delay));
            enigo.key_up(key);
            sleep(Duration::from_millis(up_press_delay));
        }

        enigo.key_up(Key::Control);
    } else {
        let mut arrows: bool = false;
        let second_arg = if let Some(arg) = args.get(2) {
            arg.as_str()
        } else {
            "none"
        };

        let third_arg = if let Some(arg) = args.get(3) {
            arg.as_str()
        } else {
            "none"
        };
        if second_arg == "arrows" || third_arg == "arrows" {
            arrows = true;
        }
        if second_arg == "no_ctrl" || third_arg == "no_ctrl" {
            for c in command.chars() {
                let down_press_delay: u64 = rng.gen_range(lower_down..upper_down);
                let up_press_delay: u64 = rng.gen_range(lower_up..upper_up);

                let key: Key;
                if !arrows {
                    key = Key::Layout(c);
                } else {
                    key = parse_key(c);
                }
                enigo.key_down(key);
                sleep(Duration::from_millis(down_press_delay));
                enigo.key_up(key);
                sleep(Duration::from_millis(up_press_delay));
            }
        } else {
            enigo.key_down(Key::Control);
            sleep(Duration::from_millis(ctrl_delay));

            for c in command.chars() {
                let down_press_delay: u64 = rng.gen_range(lower_down..upper_down);
                let up_press_delay: u64 = rng.gen_range(lower_up..upper_up);
                let key: Key;
                if !arrows {
                    key = Key::Layout(c);
                } else {
                    key = parse_key(c);
                }
                enigo.key_down(key);
                sleep(Duration::from_millis(down_press_delay));
                enigo.key_up(key);
                sleep(Duration::from_millis(up_press_delay));
            }

            enigo.key_up(Key::Control);
        }
    }
}

fn parse_key(key: char) -> enigo::Key {
    match key {
        'w' => enigo::Key::UpArrow,
        's' => enigo::Key::DownArrow,
        'a' => enigo::Key::LeftArrow,
        'd' => enigo::Key::RightArrow,
        'c' => enigo::Key::Control,
        _ => enigo::Key::Layout(key),
    }
}
