# Helldivers 2 Stratagem Macro Engine For Streamdeck

[![Join Our Discord!](https://img.shields.io/badge/Join%20Our%20Discord!-5865F2?style=flat&logo=discord&logoColor=white&link=https://discord.gg/FWGKzpaszP)](https://discord.gg/FWGKzpaszP)

This program contains macros for various stratagems to use. All macros are contained in the macro_engine.exe program, and you can specify which macro is the one that runs by adding the name of the stratagem as an argument when launching the program. 

![streamdeck](https://cdn.tokyo.jp/DlEdj)

## Streamdeck How-To

* First, we need to install this plugin from the elgato marketplace: https://marketplace.elgato.com/product/advanced-launcher-d9a289e4-9f61-4613-9f86-0069f5897125
* Once installed, drag in an "Advanced launcher" from this plugin.
* Then, you need to download the .exe file from the releases tab in this repository: https://github.com/TannerReynolds/Helldivers2-Macro-Engine/releases
    * *Alternatively, you can choose to build the exe from the source code yourself.*
* Once downloaded, set the "Application" to the "macro_engine.exe" you just downloaded.
* Check the box at the bottom that says "Run in background" (required to work in-game)
* And finally, type in the stratagem name in the "arguments" section

### Additional Options
* Arrow Key Support
    * If you would like to use arrow keys, just put "arrows" **after** the first argument. Example: `hellbomb arrows`
* No Control/Holding down the key for you
    * If you would like to press or hold the strategem menu key down yourself, use `no_ctrl` **after** the first argument. You can use this alongside `arrows` as well. Example 1: `hellbomb no_ctrl`, Example 2: `hellbomb arrows no_ctrl`

*The recommended setup is to change your bind for bringing up the strategem menu, so that you only need to press it, rather than hold it. Then, you want to use the `no_ctrl` option, and press the button yourself before using a macro. I have found this to be the most reliable method of getting macros working.*

*Keep in mind: This method is not 100%, the failure rate is actually pretty high, regardless of what the key press timings are. I am not sure why.*

*Your anti virus might flag this program, this is because the author (me) is unknown, and because this program sends keyboard inputs on your behalf.*

**Join our Discord server for the images you can use for your own Streamdeck!**

## Current List Of Supported Stratagems
#### MUST BE TYPED IN ARGUEMENTS BOX EXACTLY LIKE SHOWN BELOW
```
machine_gun_sentry
gatling_sentry
mortar_sentry
guard_dog
autocannon_sentry
rocket_sentry
ems_mortar_sentry
anti-personnel_minefield
supply_pack
grenade_launcher
laser_cannon
incendiary_mines
guard_dog_rover
ballistic_shield_backpack
arc_thrower
shield_generator_pack
orbital_precision_strike
orbital_gas_strike
orbital_ems_strike
orbital_smoke_strike
hmg_emplacement
shield_generation_relay
tesla_tower
eagle_strafing_run
eagle_airstrike
eagle_cluster_bomb
eagle_napalm_airstrike
jump_pack
eagle_smoke_strike
eagle_110mm_rocket_pods
eagle_500kg_bomb
orbital_gatling_barrage
orbital_airburst_strike
orbital_120mm_he_barrage
orbital_380mm_he_barrage
orbital_walking_barrage
orbital_lasers
orbital_railcannon_strike
machine_gun
anti-material_rifle
stalwart
expendable_anti-tank
recoiled_rifle
flamethrower
autocannon
railgun
spear
sos_beacon
resupply
hellbomb
reinforce
```
