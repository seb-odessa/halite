#![allow(non_snake_case)]
#![allow(warnings)]

#[macro_use] extern crate text_io;

mod hlt;
use hlt::{ networking, types };
use hlt::types::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let (id, map) = networking::get_init();
    let mut bot = SmartBot::new(id, map, "Smart v11 Bot");
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
}
impl SmartBot {

    pub fn new<T: Into<String>>(id: u8, map: GameMap, name: T) -> Self {
        SmartBot {id: id, map: map, name: name.into()}
    }

    pub fn get_init(&self) -> String {
        format!("{} {}", &self.name, &self.id)
    }

    pub fn get_map<'a>(&'a mut self) -> &'a mut GameMap {
        &mut self.map
    }

    fn negative(&self, mv: u8) -> u8 {
        match mv {
            WEST => EAST,
            EAST => WEST,
            NORTH => SOUTH,
            SOUTH => NORTH,
            _ => STILL
        }
    }

    pub fn get_moves(&mut self) -> HashMap<Location, u8> {
        let mut disabled = HashSet::new();
        let mut moves = HashMap::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let l = Location { x: x, y: y };
                let mv = self.calculate_moves(l);
                if !disabled.contains(&(l, mv)) {
                    moves.insert(l, mv);
                    disabled.insert((self.next(l,mv), self.negative(mv)));
                } else {
                    moves.insert(l, STILL);
                }
            }
        }
        return moves;
    }

    fn site(&self, l: Location, dir: u8) -> Site {
        self.map.get_site_ref(l, dir).clone()
    }

    fn next(&self, l: Location, dir: u8) -> Location {
        self.map.get_location(l, dir).clone()
    }

    fn distance(&mut self, l: Location, d: u8) -> u16 {
        let mut cnt = 1;
        let mut loc = l;
        while self.id == self.site(loc, d).owner {
            cnt += 1;
            loc = self.next(loc, d);
            if loc == l {
                break;
            }
        }
        return cnt;
    }

    fn is_good(&self, site: &Site, target: &Site) -> bool {
        (site.owner == target.owner) && (site.strength as i16 + target.strength as i16 <= 270)
    }

    fn best_relocation(&mut self, l: Location) -> Option<u8> {
        let site = self.site(l, STILL);
        if site.strength > 32 {
            let mut weights: Vec<(u16, u8)> = CARDINALS.iter().map(|d|{
                (self.distance(l, *d), *d)
            }).collect();
            weights.sort_by(|a, b| a.0.cmp(&b.0));
            if let Some(tuple) = weights.first() {
                let target = self.site(l, tuple.1);
                if self.is_good(&site, &target)  {
                    return Some(tuple.1);
                }
            }
        }
        return None;
    }

    fn best_attack(&mut self, l: Location) -> Option<u8> {
        let site = self.site(l, STILL);
        let mut w: Vec<(i16, u8)> = CARDINALS.iter().map(|d|{
            let target = self.site(l, *d);
            if site.owner != target.owner && site.strength > target.strength {
                (site.strength as i16 - target.strength as i16 + target.production as i16, *d)
            } else {
                (0, STILL)
            }
        }).collect();
        w.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some(tuple) = w.last() {
            if STILL != tuple.1 {
                return Some(tuple.1);
            }
        }
        None
    }

    fn best_improve(&mut self, l: Location) -> Option<u8> {
        let site = self.site(l, STILL);
        if site.strength < 6 * site.production {
            return None
        }
        let mut w: Vec<(i16, u8)> = CARDINALS.iter().map(|d|{
            let target = self.site(l, *d);
            if site.strength < target.strength && self.is_good(&site, &target) {
                (site.strength as i16 + target.strength as i16, *d)
            } else {
                (site.strength as i16, STILL)
            }
        }).collect();
        w.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some(tuple) = w.last() {
            if STILL != tuple.1 {
                return Some(tuple.1);
            }
        }
        None
    }

    fn calculate_moves(&mut self, l: Location) -> u8 {
        let site = self.site(l, STILL);
        if site.owner == self.id {
            if let Some(attack) = self.best_attack(l) {
                return attack;
            }
            if let Some(relocation) = self.best_relocation(l) {
                return relocation;
            }
            if let Some(improvement) = self.best_improve(l) {
                return improvement;
            }
        }
        return STILL;
    }
}

