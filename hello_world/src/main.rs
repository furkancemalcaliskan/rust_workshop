fn main() {
    //variable_practice

    let name = String::from("Furkan Cemal Çalışkan");
    println!("Hello, {name}");

    let mut player_score = 51; //Integer
    player_score += 1;

    println!("Player score {player_score}");

    let delta_time = 1.25;
    let delta_time: f32 = 1.25;
    let delta_time = 1.25f32;
    let delta_time = 1.25_f32;

    println!("Current delta time {delta_time}");

    let total_points: u8 = 1 + 2 + 5;

    let color_in_hex = 0xFF0033;
    println!("Background color is {color_in_hex}");
    println!("Background color is {color_in_hex:x}");

    let dir_permission = 0o755;

    println!("Directory permission {dir_permission}");
    println!("Directory permission {dir_permission:o}");
    let salary = 1_234_000_233;
    let gate_flag: u8 = 0b1010_0100;

    println!("Salary {salary}");
    println!("Gate flag {gate_flag}");

    let is_active = true;
    println!("Is active {is_active}");

    let first_char = 'a';
    println!("First char {first_char}");

    let config = (640, 480, String::from("Main Title"), false);
    println!("The config is {config:?}");

    let width = config.0;
    let height = config.1;
    let (w, h) = (width, height);
    println!("The screen resolution is {w}:{h}");

    let mut scores:[u8;5] = [56,10,90,100,48];
    println!("The scores are {scores:?}");
    println!("First score is {}. Length is {}", scores[0], scores.len());
    scores[1] += 10;

    println!("Background color is {BACKGROUND_COLOR:?}");
}

const BACKGROUND_COLOR:(u8, u8, u8) = (0xff, 0xff, 0xff);