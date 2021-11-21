use crate::{Input, State};
use generational_arena::Index;
use hostess::{Bincoded};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum CustomMsg {
    ServerSnapshotFull {
        /// the timestamp of the last input recv and processed by the server
        input_timestamp_sec:f64,
        state:State
    },
    ServerSnapshotDelta {
        /// the timestamp of the last input recv and processed by the server
        input_timestamp_sec:f64,
        delta:Vec<u8>
    },
    ServerPlayerInfo {
        thing_id:Option<Index>,
        tick_rate:u8
    },

    /// input from a client, such as position, ability usage, e.g.
    ClientInput {
        input:Input
    }
}


impl Bincoded for CustomMsg {
    
}
