use rand::Rng;


#[derive(Copy, Clone)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

const CARD_POS: [Pos; 4] = [
    Pos{x: 0, y: -1},
    Pos{x: 0, y: 1},
    Pos{x: 1, y: 0},
    Pos{x: -1, y: 0}
];

fn create_map(size: Pos) -> Vec<Vec<char>> {
    let array_height: usize = (size.y * 2 + 1) as usize;
    let array_width: usize = (size.x * 2 + 1) as usize;

    let row: Vec<char> = vec!['X'; array_width];
    let mut map: Vec<Vec<char>> = vec![row; array_height];

    for i in (1..array_height).step_by(2) {
        for j in (1..array_width).step_by(2) {
            map[i][j] = '\0';
        }
    }

    map[1][0] = 'S';
    map[array_height - 2][array_width - 1] = 'F';

    return map;
}

fn check_pos(size: Pos, position: Pos) -> bool {
    if position.x < 0 || position.x >= size.x {
        return false;
    }
    if position.y < 0 || position.y >= size.y {
        return false;
    }
    return true;
}

fn get_ways(map: Vec<Vec<char>>, size: Pos, killer: &mut Pos) -> Vec<usize> {
    let mut ways: Vec<usize> = vec![];
    let mut new_way: Pos = killer.clone();

    for i in 0..4 {
        new_way.x += CARD_POS[i].x;
        new_way.y += CARD_POS[i].y;
        if check_pos(size, new_way) {
            if map[(new_way.y * 2 + 1) as usize][(new_way.x * 2 + 1) as usize] != '.' {
                ways.push(i);
            }
        }
        new_way = killer.clone();
    }

    return ways;
}

fn get_connect_way(map: Vec<Vec<char>>, size: Pos, killer: Pos) -> Vec<usize> {
    let mut ways: Vec<usize> = vec![];
    let mut new_way: Pos = killer.clone();

    for i in 0..4 {
        new_way.x += CARD_POS[i].x;
        new_way.y += CARD_POS[i].y;
        if check_pos(size, new_way) {
            if map[(new_way.y * 2 + 1) as usize][(new_way.x * 2 + 1) as usize] == '.' {
                ways.push(i);
            }
        }
        new_way = killer.clone();
    }

    return ways;
}

fn make_way(map: &mut Vec<Vec<char>>, killer: &mut Pos, direction: usize) {
    killer.x += CARD_POS[direction].x;
    killer.y += CARD_POS[direction].y;

    let map_x_coord: i32 = killer.x * 2 + 1 - CARD_POS[direction].x;
    let map_y_coord: i32 = killer.y * 2 + 1 - CARD_POS[direction].y;

    map[map_y_coord as usize][map_x_coord as usize] = '.';
    map[(killer.y * 2 + 1) as usize][(killer.x * 2 + 1) as usize] = '.';
}

fn kill(map: &mut Vec<Vec<char>>, size: Pos, killer: &mut Pos) {
    let mut rng = rand::thread_rng();
    let mut ways: Vec<usize> = get_ways(map.clone(), size, killer);
    let mut dir: usize;

    map[(killer.y * 2 + 1) as usize][(killer.x * 2 + 1) as usize] = '.';
    while ways.len() > 0 {
        dir = rng.gen_range(0..ways.len());
        make_way(map, killer, ways[dir]);
        ways = get_ways(map.clone(), size, killer);
    }
}

fn hunt_free_tile(map: &mut Vec<Vec<char>>, size: Pos) -> Pos {
    let array_height: usize = (size.y * 2 + 1) as usize;
    let array_width: usize = (size.x * 2 + 1) as usize;
    let mut rng = rand::thread_rng();
    let mut ways: Vec<usize>;
    let dir: usize;
    let mut new_killer: Pos;

    for i in (1..array_height).step_by(2) {
        for j in (1..array_width).step_by(2) {
            new_killer = Pos{x: ((j - 1) / 2) as i32, y: ((i - 1) / 2 ) as i32};
            if map[i][j] == '\0' {
                ways = get_connect_way(map.clone(), size, new_killer);
                if ways.len() > 0 {
                    dir = rng.gen_range(0..ways.len());
                    make_way(map, &mut new_killer, ways[dir]);
                    return Pos{x: ((j - 1) / 2) as i32, y: ((i - 1) / 2 ) as i32};
                }
            }
        }
    }
    return Pos{x: -1, y: -1};
}

pub fn generator(size: Pos) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = create_map(size);
    let mut rng = rand::thread_rng();
    let mut killer: Pos = Pos{x: rng.gen_range(0..size.x), y: rng.gen_range(0..size.y)};

    while killer.x >= 0 {
        kill(&mut map, size, &mut killer);
        killer = hunt_free_tile(&mut map, size);
    }

    return map;
}