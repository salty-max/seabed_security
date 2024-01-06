use std::io;

#[derive(Debug, Default)]
pub struct Drone {
    pub id: usize,
    pub x: i32,
    pub y: i32,
    pub emergency: u8,
    pub battery: u8,
    pub is_foe: bool,
    pub radar_blips: Vec<RadarBlip>,
    pub scans: Vec<usize>,
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
            radar_blips: vec![],
            scans: vec![],
        }
    }

    pub fn move_direction(&self, dir: &str) -> [i32; 2] {
        match dir {
            "TL" => [self.x - 600, self.y - 600],
            "TR" => [self.x + 600, self.y - 600],
            "BL" => [self.x - 600, self.y + 600],
            "BR" => [self.x + 600, self.y + 600],
            _ => [self.x, self.y + 300],
        }
    }
}

#[derive(Debug)]
pub struct RadarBlip {
    pub creature_id: usize,
    pub direction: String,
}
