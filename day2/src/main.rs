use std::fs;

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Debug, Clone, Copy)]
struct Hand {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Hand {
    pub fn new() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }

    pub fn from_string(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;

        for block in input.trim().split(", ") {
            let (number,color) = block.split_once(" ").unwrap();
            match color {
                "red" => red = number.parse()?,
                "green" => green = number.parse()?,
                "blue" => blue = number.parse()?,
                _ => continue,
            }
        }

        Ok( Self { red, green, blue } )
    }

    pub fn is_valid(&self) -> bool {
        return !(self.red > MAX_RED || self.green > MAX_GREEN || self.blue > MAX_BLUE)
    }
    
    /// increments values based on highest seen value
    pub fn update_size(&mut self, other: Self) {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
    }
}

fn is_game_valid(game: &str) -> (bool, Hand) {
    let mut is_valid = true;
    let mut max_size = Hand::new();

    for hand_str in game.trim().split(";") {
        let hand = Hand::from_string(hand_str.trim()).unwrap();
        if !hand.is_valid() {
            is_valid = false
        }
        max_size.update_size(hand);
    }
    return (is_valid, max_size);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input")?;

    let mut sum: usize = 0;
    let mut power_sum = 0;

    let games = input.split('\n');
    
    for (i, game) in games.enumerate() {
        if i == 100 {
            break;
        }
        let (id_str, game) = game.trim().split_once(": ").unwrap();
        let id: usize = id_str.trim_start_matches("Game ").parse()?;

        let (is_valid, hand_size) = is_game_valid(game);
        if is_valid {
            sum += id;
        }
        power_sum += hand_size.red * hand_size.green * hand_size.blue;
    }

    println!("sum: {}", sum);
    println!("power sum: {}", power_sum);

    Ok(())
}
