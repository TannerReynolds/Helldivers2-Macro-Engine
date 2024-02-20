extern crate enigo;

use enigo::*;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
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
        "railgun" => "sdaswad",
        "spear" => "sswss",
        "sos_beacon" => "wsaw",
        "resupply" => "sswd",
        "hellbomb" => "swaswdsw",
        _ => {
            println!("Unknown argument: {}", args[1]);
            return;
        }
    };

    run_macro(command);
}

fn run_macro(command: &str) {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::LControl);
    sleep(Duration::from_millis(80));

    for c in command.chars() {
        let key = Key::Layout(c);
        enigo.key_down(key);
        sleep(Duration::from_millis(30));
        enigo.key_up(key);
        sleep(Duration::from_millis(80));
    }

    enigo.key_up(Key::LControl);
}
