/**
 * -----------------------------------------------------------------------------
 * COMMAND PATTERN
 *
 * To execute, please run: cargo run --bin command
 * To run tests, please run: cargo test --bin command
 * -----------------------------------------------------------------------------
 *
 *
 * Command pattern is a behavioral design pattern that turns a request into a
 * stand-alone object that contains all the information about the request.
 *
 * Key components in this pattern are as follows:
 *
 * 1. Command   : Trait that defines the common method
 * 2. Concrete Command  : Each struct that implements Command trait
 * 3. Invoker   : function that invokes the concrete command
 * 4. Receiver  : Function that performs actual work
 * 5: Client    : Part of a code that uses the above mechanism
 *
 * Example below uses Command pattern to fire a weapon in a shooting game in
 * which firing command directly fires any weapon without knowing the detail
 * about each element's fire procedure.
 */
use console::Term;

// Define Command
trait Command {
    fn execute(&self) -> u8;
}

// Define Weapon Classes and traits

trait WeaponTrait {
    fn fire(&mut self) -> u8; // it returns number of bullets fired
}

#[derive(Clone)]
struct AssultRifle {
    fire_rate: u8,
}

#[derive(Clone)]
struct Sniper {
    fire_rate: u8,
    efficiency: u8,
}

impl WeaponTrait for AssultRifle {
    fn fire(&mut self) -> u8 {
        self.fire_rate
    }
}

impl WeaponTrait for Sniper {
    fn fire(&mut self) -> u8 {
        (self.fire_rate as f32 * self.efficiency as f32 * 0.01) as u8
    }
}

// Define Concrete Command classes and implementations

#[derive(Clone)]
struct FireAssultRifleCommand {
    weapon: AssultRifle,
}

struct FireSniperCommand {
    weapon: Sniper,
}

impl Command for FireAssultRifleCommand {
    fn execute(&self) -> u8 {
        self.weapon.clone().fire()
    }
}

impl Command for FireSniperCommand {
    fn execute(&self) -> u8 {
        self.weapon.clone().fire()
    }
}

fn main() {
    let assult_rifle = AssultRifle { fire_rate: 20 };
    let sniper = Sniper {
        fire_rate: 10,
        efficiency: 90,
    };

    let fire_assult_rifle_command = FireAssultRifleCommand {
        weapon: assult_rifle,
    };
    let fire_sniper_command = FireSniperCommand { weapon: sniper };

    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(fire_assult_rifle_command),
        Box::new(fire_sniper_command),
    ];

    let mut selected = 0;
    let mut ammo = 50;

    let stdout = Term::stdout();
    loop {
        stdout.clear_screen().unwrap();
        println!(
            r#"
===============================================================================
[ 1 ]: Rifle     [ 2 ]: Sniper    [ F ]: Fire    [ R ]: Reload   [ Q ]: Quit
===============================================================================
[ {} ]:  {}
"#,
            if selected == 0 {
                "︻デ═一"
            } else {
                "╾━╤デ╦︻"
            },
            if ammo > 0 {
                "☢ ".repeat(ammo)
            } else {
                "!! NO AMMO !!".to_string()
            }
        );
        if let Ok(character) = stdout.read_char() {
            match character {
                '1' => {
                    if selected != 0 {
                        selected = 0;
                        ammo = 50;
                    }
                }
                '2' => {
                    if selected != 1 {
                        selected = 1;
                        ammo = 50;
                    }
                }
                'f' => {
                    if ammo > 0 {
                        let command = commands.get(selected).clone().unwrap();
                        let bullets_fired = command.execute();
                        let a = (bullets_fired as f32 / 5f32).max(0.0) as usize;
                        if a < ammo {
                            ammo -= a;
                        } else {
                            ammo = 0;
                        }
                    }
                }
                'r' => {
                    ammo = 50;
                }
                _ => {
                    break;
                }
            }
        }
    }
}
