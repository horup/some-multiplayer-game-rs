use generational_arena::Index;
use glam::Vec2;
use sample_lib::{State, Thing, move_thing_direct_sweep};

pub struct Bot {
    pub thing_id: Index,
    pub think: f64,
    pub dir: Vec2,
}

impl Bot {
    pub fn new(thing_id: Index) -> Self {
        Self {
            thing_id,
            think: 0.0,
            dir: Vec2::new(0.0, 0.0),
        }
    }
    pub fn tick(&mut self, state: &mut State, delta: f64) {
        if self.think <= 0.0 {
            if self.dir.length() == 0.0 {
                self.dir = Vec2::new(rand::random::<f32>() - 0.5, rand::random::<f32>() - 0.5).normalize();
            } else {
                self.dir = Vec2::new(0.0, 0.0);
            }
            self.think = rand::random::<f64>() * 0.5 + 0.5;
        }

        // how to avoid clone?
        let cloned = state.clone();
        if let Some(thing) = state.things.get_mut(self.thing_id) {
            if let Thing::Player(player) = thing {
                if player.is_alive() {
                    let v = self.dir;
                    let v = v * player.speed * delta as f32;
                    let new_pos = player.pos + v;
                    move_thing_direct_sweep((self.thing_id, thing), new_pos, &cloned, None);
                }
            }
           
        }

        self.think -= delta;
    }
}
