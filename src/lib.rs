mod command;
mod creature;
mod drone;

use std::{collections::HashMap, io};

use command::Command;
use creature::Creature;
use drone::Drone;

#[derive(Debug)]
pub struct GameState {
    creatures: HashMap<usize, Creature>,
    my_score: u8,
    foe_score: u8,
    my_drones: Vec<Drone>,
    foe_drones: Vec<Drone>,
    scans: HashMap<usize, usize>,
}

impl GameState {
    pub fn init() -> Self {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let creature_count = input_line.trim().parse().unwrap();
        let mut creatures = HashMap::new();

        for _ in 0..creature_count {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.trim().split(' ').collect::<Vec<_>>();
            let creature: Creature = inputs.into();
            creatures.insert(creature.id, creature);
        }

        Self {
            creatures,
            my_score: 0,
            foe_score: 0,
            my_drones: vec![],
            foe_drones: vec![],
            scans: HashMap::new(),
        }
    }

    pub fn turn(&mut self) {
        self.get_input();
        self.my_drones
            .iter()
            .for_each(|_| println!("{}", Command::Wait { light: false }));
    }

    fn get_input(&mut self) {
        self.my_score = self.get_u8();
        self.foe_score = self.get_u8();

        let my_scan_count = self.get_u8();
        let my_scans = self.get_ids(my_scan_count);
        let foe_scan_count = self.get_u8();
        let foe_scans = self.get_ids(foe_scan_count);

        self.scan_creatures(my_scans, true);
        self.scan_creatures(foe_scans, false);

        let my_drone_count = self.get_u8();
        for _ in 0..my_drone_count {
            let drone = Drone::new_from_input(false);
            if let Some(d) = self.my_drones.get_mut(drone.id) {
                *d = drone;
            } else {
                self.my_drones.push(drone);
            }
        }

        let foe_drone_count = self.get_u8();
        for _ in 0..foe_drone_count {
            let drone = Drone::new_from_input(true);
            if let Some(d) = self.foe_drones.get_mut(drone.id) {
                *d = drone;
            } else {
                self.foe_drones.push(drone);
            }
        }

        let drone_scan_count = self.get_u8();
        for _ in 0..drone_scan_count {
            let input_line = self.get_input_line();
            let inputs = input_line.trim().split(' ').collect::<Vec<_>>();
            let drone_id = inputs[0].parse().unwrap();
            let creature_id = inputs[1].parse().unwrap();

            self.scans.insert(drone_id, creature_id);
        }

        let visible_creature_count = self.get_u8();
        for _ in 0..visible_creature_count {
            let input_line = self.get_input_line();
            let inputs = input_line.trim().split(' ').collect::<Vec<_>>();
            let creature_id = inputs[0].parse().unwrap();
            let creature_x = inputs[1].parse().unwrap();
            let creature_y = inputs[2].parse().unwrap();
            let creature_vx = inputs[3].parse().unwrap();
            let creature_vy = inputs[4].parse().unwrap();

            let Some(creature) = self.creatures.get_mut(&creature_id) else {
                eprintln!("Missing creature when updating info: {creature_id}");
                continue;
            };

            creature.x = creature_x;
            creature.y = creature_y;
            creature.vx = creature_vx;
            creature.vy = creature_vy;
        }

        let radar_blip_count = self.get_u8();
        for _ in 0..radar_blip_count {
            let input_line = self.get_input_line();
            let inputs = input_line.trim().split(' ').collect::<Vec<_>>();
            let drone_id = inputs[0];
            let creature_id = inputs[1];
            let radar = inputs[2];
        }
    }

    fn get_input_line(&self) -> String {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line
    }

    fn get_u8(&self) -> u8 {
        self.get_input_line().trim().parse().unwrap()
    }

    fn get_ids(&self, count: u8) -> Vec<usize> {
        let mut ids = vec![];

        for _ in 0..count {
            ids.push(self.get_u8() as usize);
        }

        ids
    }

    fn scan_creatures(&mut self, ids: Vec<usize>, is_my_scan: bool) {
        for id in ids {
            let Some(creature) = self.creatures.get_mut(&id) else {
                eprintln!("trying to mark unknown creature: {id}");
                continue;
            };

            if is_my_scan {
                creature.my_scan = true;
            } else {
                creature.foe_scan = true;
            }
        }
    }
}
