use std::io;

#[derive(Debug, Default)]
pub struct Drone {
    pub id: usize,
    pub x: u32,
    pub y: u32,
    pub emergency: u8,
    pub battery: u8,
    pub is_foe: bool,
}

impl Drone {
    pub fn new_from_input(is_foe: bool) -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.trim().split(' ').collect::<Vec<_>>();

        let id = inputs[0].parse().unwrap();
        let x = inputs[1].parse().unwrap();
        let y = inputs[2].parse().unwrap();
        let emergency = inputs[3].parse().unwrap();
        let battery = inputs[4].parse().unwrap();

        Self {
            id,
            x,
            y,
            emergency,
            battery,
            is_foe,
        }
    }
}
