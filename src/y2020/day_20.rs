use std::collections::BTreeMap;

struct Tile {
    pub id : usize,
    pub data : [[bool;10];10],
}
impl Tile {
    pub fn new(id : usize) -> Tile {
        Tile{
            id,
            data: [[false;10];10]
        }
    }
}

#[derive(Copy, Clone)]
struct RotatedTile<'a> {
    pub tile : &'a Tile,
    pub orientation : Orientation,
}
impl<'a> RotatedTile<'a> {
    fn get(&self, y : usize, x: usize) -> bool {
        match self.orientation {
            O0 => self.tile.data[y][x],
            O1 => self.tile.data[y][9-x],
            O2 => self.tile.data[9-y][x],
            O3 => self.tile.data[9-y][9-x],
            O4 => self.tile.data[x][y],
            O5 => self.tile.data[x][9-y],
            O6 => self.tile.data[9-x][y],
            O7 => self.tile.data[9-x][9-y],
        }
    }
    pub fn can_be_left_right(&self, right: &RotatedTile<'a> ) -> bool {
        for y in 0..10 {
            if self.get(y, 9) != right.get(y, 0) {
                return false;
            }
        }
        true
    }

    pub fn can_be_top_bottom(&self, bottom: &RotatedTile<'a> ) -> bool {
        for x in 0..10 {
            if self.get(9, x) != bottom.get(0, x) {
                return false;
            }
        }
        true
    }
}

#[derive(Copy, Clone)]
enum Orientation {
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
}
use Orientation::*;

impl Orientation {
    pub fn all() -> [Orientation;8] {
        [O0, O1, O2, O3, O4, O5, O6, O7]
    }
}

fn tiles_from_str(str: &str) -> BTreeMap<usize, Tile> {
    let mut tiles = BTreeMap::new();

    let mut lines = str.lines();
    loop {
        let line = lines.next();
        if line.is_none() {
            return tiles;
        }
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let id = line.split_at(line.len() - 1).0.split_at(5).1.parse().unwrap();
        let mut tile = Tile::new(id);
        for y in 0..10 {
            let line = lines.next().unwrap().as_bytes();
            for x in 0..10 {
                tile.data[y][x] = line[x] == '#' as u8;
            }
        }
        tiles.insert(id, tile);
    }
}

pub fn solve(){
    const INPUT : &str =  include_str!("../../inputs/2020/20");

    let tiles = tiles_from_str(INPUT);

    let sorted = sort_tiles(&tiles);
    let corners = sorted[y_x_to_pos(0,0)].tile.id
        * sorted[y_x_to_pos(0, 11)].tile.id
        * sorted[y_x_to_pos(11, 11)].tile.id
        * sorted[y_x_to_pos(11, 0)].tile.id;

    println!("[PART 1] Product of corners {}", corners);

    let mut image = [[false;8*12];8*12];
    let mut x = 0;
    let mut y = 0;
    for oy in 0..12 {
        for iy in 1..9 {
            for ox in 0..12 {
                for ix in 1..9 {
                    image[y][x] = sorted[y_x_to_pos(oy, ox)].get(iy, ix);
                    x +=1;
                }
            }
            y += 1;
            x = 0;
        }
    }
    let sea_monsters = count_sea_monsters(&image);
    let rough = image
        .iter()
        .map(|r| r.iter().filter(|p| **p).count())
        .sum::<usize>() - sea_monsters * 15;

    println!("[PART 2] Rough water {}", rough);
}

fn count_sea_monsters(image: &[[bool;96];96]) -> usize {
    fn get(image: &[[bool;96];96], o: Orientation, y : usize, x: usize) -> bool {
        match o {
            O0 => image[y][x],
            O1 => image[y][95-x],
            O2 => image[95-y][x],
            O3 => image[95-y][95-x],
            O4 => image[x][y],
            O5 => image[x][95-y],
            O6 => image[95-x][y],
            O7 => image[95-x][95-y],
        }
    }
    let mut sea_monsters = 0;
    for o in Orientation::all().iter() {
        for y in 0..=96 - 3 {
            for x in 0..=96 - 20 {
                if get(image, *o, y+1, x+0)
                    && get(image, *o, y+1, x+0)
                    && get(image, *o, y+2, x+1)
                    && get(image, *o, y+2, x+4)
                    && get(image, *o, y+1, x+5)
                    && get(image, *o, y+1, x+6)
                    && get(image, *o, y+2, x+7)
                    && get(image, *o, y+2, x+10)
                    && get(image, *o, y+1, x+11)
                    && get(image, *o, y+1, x+12)
                    && get(image, *o, y+2, x+13)
                    && get(image, *o, y+2, x+16)
                    && get(image, *o, y+1, x+17)
                    && get(image, *o, y+1, x+18)
                    && get(image, *o, y+0, x+18)
                    && get(image, *o, y+1, x+19) {
                    sea_monsters += 1;
                }
            }
        }
    }
    sea_monsters
}

fn sort_tiles(tiles: &BTreeMap<usize, Tile>) -> Vec<RotatedTile> {
    find_all(tiles, Vec::new()).unwrap()
}

fn find_all<'a>(tiles: &'a BTreeMap<usize, Tile>,
                used: Vec<RotatedTile<'a>>) -> Option<Vec<RotatedTile<'a>>> {
    if used.len() == 144 {
        return Some(used);
    }
    for p in find_tile(tiles, &used) {
        let mut used = used.clone();
        used.push(p);
        match find_all(tiles, used) {
            None => {},
            Some(p) => {
                return Some(p);
            }
        }
    }

    None
}

fn find_tile<'a>(tiles: &'a BTreeMap<usize, Tile>,
                 used: &[RotatedTile<'a>]) -> Vec<RotatedTile<'a>> {
    let pos = used.len();
    let (y, x) = pos_to_y_x(pos);
    let mut possibilities = Vec::new();
    for (_, t) in tiles.iter()
        .filter(|(t, _)| used.iter().all(|rt| rt.tile.id != **t)) {
        for &orientation in Orientation::all().iter() {
            let rt = RotatedTile{tile: t, orientation};
            if x > 0 {
                let left = &used[y_x_to_pos(y, x-1)];
                if !left.can_be_left_right(&rt) {
                    continue;
                }
            }
            if y > 0 {
                let top = &used[y_x_to_pos(y - 1, x)];
                if !top.can_be_top_bottom(&rt) {
                    continue
                }
            }
            possibilities.push(rt);
        }
    }

    possibilities
}

fn pos_to_y_x(pos: usize) -> (usize, usize) {
    (pos / 12, pos % 12)
}
fn y_x_to_pos(y: usize, x: usize) -> usize {
    y * 12 + x
}

