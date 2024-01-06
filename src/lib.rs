mod command;
mod creature;
mod drone;

use std::{collections::HashMap, io};

use command::Command;
use creature::Creature;
use drone::{Drone, RadarBlip};

#[derive(Debug)]
pub struct GameState {
    turns: usize,
    creatures: HashMap<usize, Creature>,
    my_score: u8,
    foe_score: u8,
    my_drones: Vec<Drone>,
    foe_drones: Vec<Drone>,
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
            turns: 0,
            creatures,
            my_score: 0,
            foe_score: 0,
            my_drones: vec![],
            foe_drones: vec![],
        }
    }

    pub fn turn(&mut self) -> Vec<Command> {
        self.turns += 1;
        self.get_input();

        let light = self.turns % 10 == 0;
        // let light = false;

        let (command, target_id) = 'drone_1_command: {
            let drone = &self.my_drones[0];
            if !drone.scans.is_empty() {
                break 'drone_1_command (
                    Command::Move {
                        x: drone.x,
                        y: 500,
                        light,
                    },
                    None,
                );
            }

            for blip in drone.radar_blips.iter() {
                let Some(creature) = self.creatures.get(&blip.creature_id) else {
                    let [x, y] = drone.move_direction(&blip.direction);
                    break 'drone_1_command (Command::Move { x, y, light }, None);
                };

                if !creature.my_scan && creature.c_type >= 0 {
                    let [x, y] = drone.move_direction(&blip.direction);
                    break 'drone_1_command (Command::Move { x, y, light }, Some(creature.id));
                }
            }

            (
                Command::Move {
                    x: drone.x,
                    y: 0,
                    light,
                },
                None,
            )
        };

        let command_2 = 'drone_2_command: {
            let drone = &self.my_drones[1];
            if !drone.scans.is_empty() {
                break 'drone_2_command Command::Move {
                    x: drone.x,
                    y: 500,
                    light,
                };
            }

            for blip in drone.radar_blips.iter() {
                if blip.creature_id == target_id.unwrap_or_default() {
                    continue;
                }

                let Some(creature) = self.creatures.get(&blip.creature_id) else {
                    let [x, y] = drone.move_direction(&blip.direction);
                    break 'drone_2_command Command::Move { x, y, light };
                };

                if !creature.my_scan && creature.c_type >= 0 {
                    let [x, y] = drone.move_direction(&blip.direction);
                    break 'drone_2_command Command::Move { x, y, light };
                }
            }

            Command::Move {
                x: drone.x,
                y: 0,
                light,
            }
        };

        vec![command, command_2]
    }

    fn get_input(&mut self) {
        self.my_score = self.get_u8();
        self.foe_score = self.get_u8();

        let _my_scan_count = self.get_u8();
        let _my_scans = self.get_ids(_my_scan_count);
        let _foe_scan_count = self.get_u8();
        let _foe_scans = self.get_ids(_foe_scan_count);

        let my_drone_count = self.get_u8();
        self.my_drones.clear();
        for _ in 0..my_drone_count {
            let drone = Drone::new_from_input(false);
            self.my_drones.push(drone);
        }

        let foe_drone_count = self.get_u8();
        self.foe_drones.clear();
        for _ in 0..foe_drone_count {
            let drone = Drone::new_from_input(true);
            self.foe_drones.push(drone);
        }

        let drone_scan_count = self.get_u8();
        for _ in 0..drone_scan_count {
            let input_line = self.get_input_line();
            let inputs = input_line.trim().split(' ').collect::<Vec<_>>();

            let drone_id = inputs[0].parse::<usize>().unwrap();
            let Some(drone) = self.my_drones.iter_mut().find(move |d| d.id == drone_id) else {
                continue;
            };

            let creature_id = inputs[1].parse::<usize>().unwrap();
            drone.scans.push(creature_id);
            self.scan_creatures(vec![creature_id], true);
        }

        self.creatures
            .iter_mut()
            .for_each(|(_, c)| c.is_visible = false);

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
                continue;
            };

            creature.x = creature_x;
            creature.y = creature_y;
            creature.vx = creature_vx;
            creature.vy = creature_vy;
            creature.is_visible = true;
        }

        let radar_blip_count = self.get_u8();

        self.my_drones
            .iter_mut()
            .for_each(|d| d.radar_blips.clear());

        for _ in 0..radar_blip_count {
            let input_line = self.get_input_line();
            let inputs = input_line
                .trim()
                .split_ascii_whitespace()
                .collect::<Vec<_>>();
            let drone_id = inputs[0].parse::<usize>().unwrap();
            let creature_id = inputs[1].parse::<usize>().unwrap();
            let direction = inputs[2].to_string();

            let Some(drone) = self.my_drones.iter_mut().find(move |d| d.id == drone_id) else {
                eprintln!("Cannot find drone {drone_id}");
                continue;
            };
            drone.radar_blips.push(RadarBlip {
                creature_id,
                direction,
            });
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

    // fn find_unscanned_creature(&self) -> Option<&Creature> {
    //     for creature in self.creatures.values() {
    //         if !creature.my_scan {
    //             return Some(creature);
    //         }
    //     }

    //     None
    // }
}
