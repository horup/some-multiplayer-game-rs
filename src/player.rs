use std::collections::VecDeque;

use generational_arena::Index;
use glam::Vec2;
use hostess::uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::State;

/// struct holding Input for a player
/// send by clients to the server
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Input {
    /// the timestamp of the input
    pub timestamp_sec:f64,

    /// the id of the thing controlled by a player owning the Input
    pub thing_id:Option<Index>,

    /// direction of the thing according to what the player believes is true
    /// (should be put into keyboard or similar outside of input)
    pub movement_dir:Vec2,

    /// how much and which direction to change the position
    pub movement:Vec2,

    /// true if the player wants to use his ability
    pub ability_trigger:bool,

    /// where the player is targeting in the world
    pub ability_target:Vec2,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub client_id:Uuid,
    pub client_name:String,
    pub thing:Option<Index>,
    pub latest_input_timestamp_sec:f64,
    pub inputs:VecDeque<Input>,
    /// last state transmitted to player
    pub state:State
}

impl Player {
   
    pub fn clear_inputs(&mut self) {
        self.inputs.clear();
    }
}