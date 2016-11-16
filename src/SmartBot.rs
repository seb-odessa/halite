#[macro_use] extern crate lazy_static;
#[macro_use] extern crate text_io;

mod hlt;
use hlt::{ networking, types };
use hlt::types::*;
use std::collections::HashMap;

fn main() {
    let (id, map) = networking::get_init();
    let mut bot = SmartBot::new(id, map, "Smart Bot");
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

    pub fn get_moves(&self) -> HashMap<Location, u8> {
        let mut moves = HashMap::new();
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

    fn calculate_moves(&self, l: Location) -> Option<u8> {
        let site = self.site(l, types::STILL);
        if site.owner == self.id {
            let mut weights: Vec<(i32, u8)> = CARDINALS.iter().map(|d|{
                let target = self.site(l, *d);
                let delta = site.strength as i32 - target.strength as i32;
                if site.owner != target.owner {
                    (delta, *d)
                } else{
                    if target.strength > 16 && site.strength > target.strength {
                        (delta, *d)
                    } else if site.strength > 200 && target.strength > 5 && site.strength > target.strength {
                        (-delta, *d)
                    } else {
                        (0, STILL)
                    }
                }


            }).collect();
            weights.sort_by(|a, b| a.0.cmp(&b.0));
            if let Some(best) = weights.last() {
                return Some(best.1);
            }

        }
        return None;
    }

}

