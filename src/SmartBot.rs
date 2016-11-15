//#![allow(non_snake_case)]
#![allow(warnings)]
extern crate rand;
#[macro_use] extern crate text_io;

mod hlt;
use hlt::{ networking, types };
use hlt::types::{Site, Location, GameMap};
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let (id, mut game_map) = networking::get_init();

    networking::send_init(format!("{}{}", "Smart Bot".to_string(), id.to_string()));
    loop {
        networking::get_frame(&mut game_map);
        let mut moves = HashMap::new();
        for y in 0..game_map.height {
            for x in 0..game_map.width {
                let l = Location { x: x, y: y };
                if let Some(mv) = get_move(id, l, &game_map) {
                    moves.insert(l, mv);
                }
            }
        }
        networking::send_frame(moves);
    }
}

fn is_good(current: &Site, possible: &Site) -> bool {
    return possible.owner != current.owner && current.strength > possible.strength;
}

fn get_move(id: u8, l: Location, map: &GameMap) -> Option<u8> {
    let site = map.get_site_ref(l, types::STILL);
    if site.owner == id {
        if is_good(&site, &map.get_site_ref(l, types::NORTH)) {
            return Some(types::NORTH);
        }
        if is_good(&site, &map.get_site_ref(l, types::SOUTH)) {
            return Some(types::SOUTH);
        }
        if is_good(&site, &map.get_site_ref(l, types::WEST)) {
            return Some(types::WEST);
        }
        if is_good(&site, &map.get_site_ref(l, types::EAST)) {
            return Some(types::EAST);
        }
        return Some(types::STILL);
    }
    None
}