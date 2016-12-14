#![allow(non_snake_case)]
#![allow(warnings)]

#[macro_use] extern crate text_io;

mod hlt;
use hlt::{ networking, types };
use hlt::types::*;
use std::collections::HashMap;


fn main() {
    let (id, map) = networking::get_init();
    let mut bot = SmartBot::new(id, map, "Smart v8 Bot");
    networking::send_init(bot.get_init());

    loop {
        networking::get_frame(&mut bot.get_map());
        networking::send_frame(bot.get_moves());
    }
}

struct SmartBot {
    id: u8,
    map: GameMap,
    name: String,
    weight: Vec<Vec<i16>>,
}
impl SmartBot {
    pub fn new<T: Into<String>>(id: u8, map: GameMap, name: T) -> Self {

        let mut w: Vec<Vec<i16>> = Vec::with_capacity(map.height as usize);
        for y in 0..map.height {
            w.push(Vec::with_capacity(map.width as usize));
            for x in 0..map.width {
                w[y as usize].push(0);
            }
        }
        SmartBot {id: id, map: map, name: name.into(), weight: w }
    }

    pub fn get_init(&self) -> String {
        format!("{} {}", &self.name, &self.id)
    }

    pub fn get_map<'a>(&'a mut self) -> &'a mut GameMap {
        &mut self.map
    }

    fn fill_weight(&mut self) {
        let mut own = HashMap::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let l = Location { x: x, y: y };
                let site = self.map.get_site_ref(l, STILL).clone();
                if site.owner != self.id {
                    self.set_weight(l, site.strength as i16);
                } else {
                    own.insert(l, 0);
                }
            }
        }
    }

    fn set_weight(&mut self, l: Location, weight: i16) {
        self.weight[l.y as usize][l.x as usize] = weight;
    }

    fn get_weight(&self, l: Location) -> i16 {
        self.weight[l.y as usize][l.x as usize]
    }

    pub fn get_moves(&mut self) -> HashMap<Location, u8> {
        let mut moves = HashMap::new();
        self.fill_weight();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let l = Location { x: x, y: y };
                if let Some(mv) = self.calculate_moves(l) {
                    moves.insert(l, mv);
                }
            }
        }
        return moves;
    }

    fn site(&self, l: Location, dir: u8) -> Site {
        self.map.get_site_ref(l, dir).clone()
    }

    fn calculate_moves(&mut self, l: Location) -> Option<u8> {
        let site = self.site(l, types::STILL);
        if site.owner == self.id {
            let mut moves: Vec<(i16, u8)> = CARDINALS.iter().map(|d|{
                let target = self.site(l, *d);
                let delta = site.strength as i16 - target.strength as i16;
                if site.owner != target.owner && site.strength >= target.strength {
                    (delta, *d)
                } else if site.owner == target.owner && site.strength < target.strength {
                    (self.get_weight(l), *d)
                } else {
                    (0, STILL)
                }



            }).collect();
            moves.sort_by(|a, b| a.0.cmp(&b.0));
            if let Some(best) = moves.last() {
                return Some(best.1);
            }
        }
        return None;
    }

}

