use generational_arena::Index;
use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::State;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Solid {
    /// not solid, does not influence its own movement nor others
    None,

    /// solid, but influences only its own movement, not others, i.e. others ignore it
    Partial,

    /// solid, all movement is influenced by this
    Solid
}

impl Default for Solid {
    fn default() -> Self {
        Self::Solid
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectileThing {
    pub pos:Vec2,
    pub radius:f32,
    pub solid:Solid,
    pub vel:Vec2,
    pub owner:Index,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PlayerThing {
    pub pos:Vec2,
    pub radius:f32,
    pub solid:Solid,
    pub hearts:i8,
    pub respawn_timer:f32,
    pub ability_cooldown:f32,
    pub speed:f32,
    pub deaths:i32,
    pub kills:i32,
    pub no_interpolation:bool,
    pub name:String,
    pub spawn_pos:Option<Vec2>
}

impl PlayerThing {
    pub fn is_alive(&self) -> bool {
        self.hearts > 0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Thing {
    Player(PlayerThing),
    Projectile(ProjectileThing)
}



impl Thing {
    pub fn new_player(name:&str) -> Self {
        Self::Player(PlayerThing {
            radius:0.5,
            speed:7.5,
            hearts:0,
            respawn_timer:3.0,
            name:name.into(),
            ..Default::default()
        })
    }

    pub fn spawn(&mut self) {
        if let Thing::Player(player) = self {
            player.pos = player.spawn_pos.unwrap_or_default();
            player.spawn_pos = None;
            player.solid = Solid::Solid;
            player.hearts = 3;
            player.respawn_timer = 0.0;
        }
    }

    pub fn new_projectile(pos:Vec2, vel:Vec2, owner:Index) -> Self {
        Self::Projectile(ProjectileThing {
            pos,
            radius:0.25,
            solid:Solid::Partial,
            owner,
            vel
        })
    }

    pub fn pos(&self) -> &Vec2 {
        match self {
            Thing::Player(t) => &t.pos,
            Thing::Projectile(t) => &t.pos,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Thing::Player(t) => t.name.as_str(),
            Thing::Projectile(_) => "",
        }
    }

    pub fn radius(&self) -> &f32 {
        match self {
            Thing::Player(t) => &t.radius,
            Thing::Projectile(t) => &t.radius,
        }
    }

    pub fn solid(&self) -> &Solid {
        match self {
            Thing::Player(t) => &t.solid,
            Thing::Projectile(t) => &t.solid,
        }
    }

    pub fn solid_mut(&mut self) -> &mut Solid {
        match self {
            Thing::Player(t) => &mut t.solid,
            Thing::Projectile(t) => &mut t.solid,
        }
    }

    pub fn pos_mut(&mut self) -> &mut Vec2 {
        match self {
            Thing::Player(t) => &mut t.pos,
            Thing::Projectile(t) => &mut t.pos,
        }
    }

    pub fn lerp_pos(&self, prev:&Thing, alpha:f32) -> Vec2 {
        let pos = *self.pos();
        let v = pos - *prev.pos();
        if v.length() < 2.0 {
            let v = v * alpha;
            return pos + v;
        }

        return pos;
    }

    pub fn no_interpolate(&self) -> bool {
        match self {
            Thing::Player(p) => p.no_interpolation,
            Thing::Projectile(_) => false,
        }
    }
}