#[derive(Debug)]
pub struct Creature {
    pub id: usize,
    pub color: i8,
    pub c_type: i8,
    pub my_scan: bool,
    pub foe_scan: bool,
    pub is_visible: bool,
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
}

impl From<Vec<&str>> for Creature {
    fn from(values: Vec<&str>) -> Self {
        let id = values[0].parse().unwrap();
        let color = values[1].parse().unwrap();
        let c_type = values[2].parse().unwrap();

        Self {
            id,
            color,
            c_type,
            my_scan: false,
            foe_scan: false,
            is_visible: false,
            x: 0,
            y: 0,
            vx: 0,
            vy: 0,
        }
    }
}
