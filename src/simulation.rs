use generational_arena::{Index};
use glam::Vec2;
use parry2d::{
    math::Isometry,
    query::{contact},
    shape::{Ball, Polyline},
};
use crate::{Event, Input, Solid, State, Thing};

pub fn apply_input(state: &mut State, input: &Input, _authorative: bool) {
    // how to avoid clone?
    let cloned = state.clone();
    if let Some(thing_id) = input.thing_id {
        if let Some(thing) = state.things.get_mut(thing_id) {
            if let Thing::Player(player) = thing {
                if player.is_alive() {
                    let new_pos = input.movement * player.speed as f32 + *thing.pos();
                    move_thing_direct_sweep((thing_id, thing), new_pos, &cloned, None);
                    clamp_to_bounds(thing, state.width, state.height);
                }
            }
        }
    }
}

pub fn update_things(state: &mut State, dt: f64) {
    // how to avoid clone
    let cloned = state.clone();

    let mut remove = Vec::new();
    let mut hits = Vec::new();

    // movement and collision handling
    for (id, thing) in state.things.iter_mut() {
        if let Thing::Player(player) = thing {
            player.ability_cooldown -= dt as f32;
            if player.ability_cooldown < 0.0 {
                player.ability_cooldown = 0.0;
            }
        }

        if let Thing::Projectile(projectile) = thing {
            let owner = projectile.owner;
            if projectile.vel.length_squared() > 0.0 {
                let new_pos = projectile.vel * dt as f32 + *thing.pos();
                let res = move_thing_direct_sweep((id, thing), new_pos, &cloned, Some(owner));

                match res {
                    CollisionResult::None => {}
                    CollisionResult::Thing(target) => {
                        remove.push(id);
                        hits.push((owner, target));
                    }
                    CollisionResult::Polyline(id, _normal) => {
                        remove.push(id);
                    }
                }
            }
        }
    }

    // hit / damage handling
    for (owner, target) in hits.drain(..) {
        if let Some(thing) = state.things.get_mut(target) {
            if let Thing::Player(player) = thing {
                if player.is_alive() {
                    player.hearts -= 1;

                    if !player.is_alive() {
                        player.respawn_timer = 3.0;
                        player.deaths += 1;
                        player.solid = Solid::None;

                        state.events.push(Event::PlayerDied {
                            thing_id: target,
                            pos: player.pos,
                        });

                        if let Some(thing) = state.things.get_mut(owner) {
                            if let Thing::Player(owner) = thing {
                                owner.kills += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // player respawn handling
    for (_id, thing) in state.things.iter_mut() {
        if let Thing::Player(player) = thing {
            if !player.is_alive() {
                if player.spawn_pos == None {
                    player.spawn_pos = Some(State::next_spawn_pos(&mut state.next_spawn, &state.map.spawn_points));
                }
                player.respawn_timer -= dt as f32;
                if player.respawn_timer <= 0.0 {
                    thing.spawn();
                }
            }
        }
    }

    // ensuring things stay within bounds
    // and remove projectiles that venture out of bounds
    for (id, thing) in state.things.iter_mut() {
        let clamped = clamp_to_bounds(thing, state.width, state.height);

        if let Thing::Projectile(_projectile) = thing {
            if clamped {
                remove.push(id);
            }
        }
    }

    // removal of entities who needs removed
    for id in remove.drain(..) {
        if let Some(thing) = state.things.remove(id) {
            if let Thing::Projectile(projectile) = thing {
                state.events.push(Event::ProjectileHit {
                    pos: projectile.pos,
                })
            }
        }
    }
}

pub fn clamp_to_bounds(thing:&mut Thing, width:f32, height:f32) -> bool {
    let pos = *thing.pos();
    let margin = 1.0;
    *thing.pos_mut() = pos.clamp(Vec2::new(margin, margin * 2.0), Vec2::new(width - margin, height - margin));

    if pos != *thing.pos() {
        return true;
    } 

    return false;
}

pub struct Circle {
    pub c: Vec2,
    pub r: f32,
}

#[derive(PartialEq)]
pub enum CollisionResult {
    None,
    Thing(Index),
    Polyline(Index, Vec2),
}

pub fn move_thing_direct_sweep(
    thing: (Index, &mut Thing),
    new_pos: Vec2,
    state: &State,
    ignore: Option<Index>,
) -> CollisionResult {
    let mut result = CollisionResult::None;
    let vel = new_pos - *thing.1.pos();
    if vel.length() > 0.0 {
        let mut dist = vel.length();
        let max_step = *thing.1.radius() / 2.0;
        let d = vel.normalize();

        let mut new_pos = *thing.1.pos();
        while dist > 0.0 {
            let step = dist.min(max_step);
            new_pos += d * step;
            result = move_thing_direct((thing.0, thing.1), new_pos, state, ignore);
            dist -= step;

            if result != CollisionResult::None {
                break;
            }

        }
    }
    result
}

fn move_thing_direct(
    thing: (Index, &mut Thing),
    new_pos: Vec2,
    state: &State,
    ignore: Option<Index>,
) -> CollisionResult {
    let mut result = CollisionResult::None;
    let (thing_id, thing1) = thing;
    *thing1.pos_mut() = new_pos;

    if *thing1.solid() != Solid::None {
        for (thing_id2, thing2) in state.things.iter() {
            // check same
            if thing_id == thing_id2 {
                continue;
            }

            // check if Solid and not just partially solid
            if *thing2.solid() != Solid::Solid {
                continue;
            }

            // check ignore
            if let Some(ignore) = ignore {
                if thing_id2 == ignore {
                    continue;
                }
            }

            let pos1: Isometry<f32> = [thing1.pos().x, thing1.pos().y].into();
            let ball1 = Ball::new(*thing1.radius());

            let pos2: Isometry<f32> = [thing2.pos().x, thing2.pos().y].into();
            let ball2 = Ball::new(*thing2.radius());

            let c = contact(&pos1, &ball1, &pos2, &ball2, 10.0);
            match c {
                Ok(res) => match res {
                    Some(res) => {
                        if res.dist < 0.0 {
                            let p: Vec2 = [res.normal1.x, res.normal1.y].into();
                            let p = p * res.dist;
                            *thing1.pos_mut() += p;
                            result = CollisionResult::Thing(thing_id2);
                            break;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        /*if result != CollisionResult::None {
            return result;
        }*/

        for (_id, p) in &state.map.polylines {
            let mut points = Vec::new();
            for p in &p.points {
                points.push([p.x, p.y].into());
            }
            if let Some(p) = points.first() {
                let p = *p;
                points.push(p);
            }

            let lines = Polyline::new(points, None);

            let pos1: Isometry<f32> = [thing1.pos().x, thing1.pos().y].into();
            let ball1 = Ball::new(*thing1.radius());
            let c = contact(&pos1, &ball1, &[0.0, 0.0].into(), &lines, 1.0);

            match c {
                Ok(res) => match res {
                    Some(res) => {
                        if res.dist < 0.0 {
                            let p: Vec2 = [res.normal1.x, res.normal1.y].into();
                            let p = p * res.dist;
                            *thing1.pos_mut() += p;
                            if result == CollisionResult::None {
                                result = CollisionResult::Polyline(thing_id, Vec2::new(res.normal2.x, res.normal2.y));
                            }
                            break;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    return result;
}
