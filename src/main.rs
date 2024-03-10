extern crate enigo;

use enigo::*;
use std::{env, string};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect(); // collect args from SD
    //setting up variables for potential arguments
    let mut press = false; 
    let mut arrows:bool = false;
    let mut delay: Option<u64> = None;
    let mut command = "";


    if args.iter().any(|arg| arg.to_lowercase() == "press".to_lowercase()) {
        press = true; //checks to see if any arguments are "press" if so sets stratagem input key to press not hold
    } else {}

    if args.iter().any(|arg| arg.to_lowercase() == "arrows".to_lowercase()) {
        arrows = true; //checks to see if any arguments are "arrows" if so sets stratagem input as arrow keys
    } else {}

    for arg in &args[1..] { //checking arguments for an integar and if found setting delay to that value
        if let Ok(d) = arg.parse::<u64>() {
            delay = Some(d);
            break;
        }
    }

    let delay = delay.unwrap_or(50); //unwrapping found delay or setting delay to 50 as default if no argument sent

    for arg in &args[1..] {
        let lowercase_arg = arg.to_lowercase(); //lowercases the argument to avoid a capital letter breaking the macro
        let matched_command = match_command(&lowercase_arg); //Checks to see if argument is vaild command
        if !matched_command.is_empty() { //if the returned value is not "" will set the stratagem 
            command = matched_command;
            break; // Break the loop if a matched command is found
        } 
    }
    run_macro(command, press, delay,arrows); //run the macro and pass over all variables 
}

fn match_command(command: &str) -> &'static str { //checks to see if passed variable is a stratagem or not
match command {
    "reinforce" => "wsdaw",
    "machine_gun_sentry" => "swddw",
    "gatling_sentry" => "swda",
    "mortar_sentry" => "swdds",
    "guard_dog" => "swawds",
    "autocannon_sentry" => "swdwaw",
    "rocket_sentry" => "swdda",
    "ems_mortar_sentry" => "swdsd",
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
    "eagle_rearm" => "wwawd",
    "hellbomb" => "swaswdsw",
    "prospecting_drill" => "ssadss",
    "super_earth_flag" => "swsw",
    "patriot_exosuit" => "asdwass",
    "seaf_artillery" => "dwws",
    "upload_data" => "adwww",
    "seismic_probe" => "wwaass",
    "orbital_illumination_flare" => "ddaa",
    _ => "", //returns blank if not so no keys will be pressed
}
}
fn run_macro(command: &str, press:bool,delay:u64,arrows:bool) {
    let mut enigo = Enigo::new(); 
    //Input key gets pressed down
    enigo.key_down(Key::Control);
    sleep(Duration::from_millis(delay));
    if press == true{ //If press is not enabled key will not be released yet
    enigo.key_up(Key::Control);
    sleep(Duration::from_millis(delay));
    }
    else{}

    for c in command.chars() {
        let mut key = Key::Layout(c);
        if arrows == true{ //if arrows enabled will pass wasd key to function to convert to arrow
            key = parse_key(c);
        }
        enigo.key_down(key);
        sleep(Duration::from_millis(delay));
        enigo.key_up(key);
        sleep(Duration::from_millis(delay));
    }
    if press == false{ //If press is not enabled, key will now be released
        enigo.key_up(Key::Control);
        sleep(Duration::from_millis(delay));
    }
    
}

fn parse_key(key: char) -> enigo::Key { //converts wasd input to arrows
    match key {
        'w' => enigo::Key::UpArrow,
        's' => enigo::Key::DownArrow,
        'a' => enigo::Key::LeftArrow,
        'd' => enigo::Key::RightArrow,
        'c' => enigo::Key::Control,
        _ => enigo::Key::Layout(key),
    }
}
