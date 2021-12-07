use std::{collections::{HashMap, VecDeque}};
use hostess::{client::Bincoded, server::{Ctx, OutMsg, InMsg, Config}, uuid::Uuid};
use sample_lib::{CustomMsg, Player, State, StateHistory, Thing, apply_input, update_things, Event};
use crate::bot::*;

pub struct Server {
    current:State,
    history:StateHistory,
    players:HashMap<Uuid, Player>,
    bots:Vec<Bot>
}

impl Default for Server {
    fn default() -> Self {
        Self {
            current:State::new(),
            players:HashMap::new(),
            bots:Vec::new(),
            history:StateHistory::new()
        }
    }
}

impl Server {
    pub fn update(&mut self, context:&mut Ctx) {
        // clear events 
        self.current.events.clear();
        self.current.timestamp = context.time;
        if self.players.len() < 2 {
            self.current.warmup = true;
            // less than two players and no bots, ensure some bots are spawned
            if self.bots.len() == 0 {
                while self.bots.len() < 4 {
                    let thing = Thing::new_player("bot");
                    let index = self.current.things.insert(thing);
                    let bot = Bot::new(index);
                    self.bots.push(bot);
                }
            }
        } else {
            self.current.warmup = false;
            // more than two players, remove bots and their things
            for bot in self.bots.drain(..) {
                self.current.things.remove(bot.thing_id);
            }

        }

        let tick_rate = TICK_RATE as u8;
        
        // process bots
        for bot in self.bots.iter_mut() {
            bot.tick(&mut self.current, context.delta);
        }
        
        // process inputs from players
        for (_, player) in &mut self.players {
            // if player has no 'thing'
            // ensure one is spawned for the player
            if player.thing == None {
                let thing = Thing::new_player(&player.client_name);
                player.thing = Some(self.current.things.insert(thing));
                // let the player know his thing id and tick_rate
                push_custom_to(context, player.client_id, CustomMsg::ServerPlayerInfo {
                    thing_id:player.thing,
                    tick_rate:tick_rate
                });
            }

            // apply input from players
            let mut trigger = false;
            for input in player.inputs.drain(..) {
                player.latest_input_timestamp_sec = input.timestamp_sec;
                let ability_target = input.ability_target;
                if input.ability_trigger {
                    trigger = true;
                }

                apply_input(&mut self.current, &input, true);

                let mut spawn = Vec::new();
                if let Some(thing_id) = player.thing {
                    if let Some(thing) = self.current.things.get_mut(thing_id) {
                        if let Thing::Player(player) = thing {
                            if player.is_alive() && trigger && player.ability_cooldown <= 0.0 {
                                player.ability_cooldown = 0.25;
                                let dir = ability_target - player.pos;
                                if dir.length() > 0.0 {
                                    let dir = dir.normalize();
                                    let v = dir * 20.0;
                                    let p = Thing::new_projectile(player.pos, v, thing_id);
                                    self.current.events.push(Event::ProjectileFired {
                                        pos:player.pos.clone()
                                    });
                                    spawn.push(p);
                                }
                            }
                        }
                    }
                }
                
                for thing in spawn.drain(..) {
                    self.current.things.insert(thing);
                }
            }
        }
        
        // do generic update of things, such as moving projectiles
        update_things(&mut self.current, context.delta);

        // for each player, transmit state diff
        for (client_id, player) in &mut self.players {
            let delta = self.current.to_delta_bincode(&player.state);
            push_custom_to(context, *client_id, CustomMsg::ServerSnapshotDelta {
                input_timestamp_sec:player.latest_input_timestamp_sec,
                delta
            });

            player.state = self.current.clone()
        }

        // remember current state
        self.history.remember(self.current.clone());
    }
}
const TICK_RATE:u64 = 20;
impl hostess::server::Server for Server {
    fn init(&mut self) -> Config {
        Config {
            tick_rate: TICK_RATE,
            max_players: 8,
        }
    }

    fn tick(&mut self, mut context:&mut Ctx) {
        while let Some(msg) = context.pop_msg() {
            match msg {
                InMsg::ClientJoined { client_id, mut client_name } => {
                    if !self.players.contains_key(&client_id) {
                        client_name.truncate(16);
                        self.players.insert(client_id, Player {
                            client_id:client_id,
                            client_name,
                            thing:None,
                            inputs:VecDeque::default(),
                            latest_input_timestamp_sec: 0.0,
                            state:self.current.clone()
                        });
                    }

                    push_custom_to(&mut context, client_id, CustomMsg::ServerSnapshotFull {
                        input_timestamp_sec:0.0,
                        state:self.current.clone()
                    });

                    push_custom_to(&mut context, client_id, CustomMsg::ServerPlayerInfo {
                        thing_id:None,
                        tick_rate:TICK_RATE as u8
                    });
                },
                InMsg::ClientLeft { client_id } => {
                    if let Some(player) = self.players.remove(&client_id) {
                        if let Some(thing_id) = player.thing {
                            self.current.things.remove(thing_id);
                        }
                    }
                },
                InMsg::CustomMsg { client_id, msg } => {
                    if let Some(msg) = Bincoded::from_bincode(&msg) {
                        self.recv_custom_msg(&mut context, client_id, msg);
                    }
                },
            }
        }

        self.update(&mut context);
    }

}

fn push_custom_to(context:&mut Ctx, client_id:Uuid, msg:CustomMsg) {
    let msg = msg.to_bincode();
    context.push_msg(OutMsg::CustomTo {
        client_id,
        msg
    });
}

impl Server {
    /// is called on each custom message received from the clients
    pub fn recv_custom_msg(&mut self, _context:&mut Ctx, client_id:Uuid, msg:CustomMsg) {
        match msg {
            CustomMsg::ClientInput { input } => {
                if let Some(player) = self.players.get_mut(&client_id) {
                    // remember input for later processing
                    player.inputs.push_back(input);
                }
            },
            _ => {}
        }
    }
}