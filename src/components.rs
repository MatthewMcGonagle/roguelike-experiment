pub mod systems;

struct Coordinates {
    x: i32,
    y: i32,
}

pub struct CoordinateComponents {
    values: [Coordinates; 4],
}

struct Timer { time: u32, reset: u32 }

impl Timer {
    fn update(&mut self) {
        self.time = self.time - 1;
        if self.time <= 0 {
            self.time = self.reset;
        }
    }
}

pub struct ActionTimers {
    values: [Timer; 4]
}

impl ActionTimers {
    pub fn update(&mut self) {
        for timer in self.values.iter_mut() {
            timer.update();
        }
    }
}

pub struct Components {
    pub coords: CoordinateComponents,
    pub action_timers: ActionTimers
}

impl Components {
    pub fn initialize() -> Components {
        Components {
            coords: CoordinateComponents {
                values: [Coordinates{x: 0, y: 0}, Coordinates{x: 200, y: 300}, Coordinates{x: 300, y: 500}, Coordinates{x: 400, y: 400}],
            },
            action_timers: ActionTimers { 
                values: [Timer { time: 10, reset: 5 }, Timer { time: 10, reset: 7 }, Timer { time: 13, reset: 13 }, Timer { time: 10, reset: 17}]
            }
        }
    }
}
