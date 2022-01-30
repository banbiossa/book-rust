use rand::Rng;

// 迷路の幅
const MAP_N: usize = 25;

fn main() {
    // get random generator
    let mut rng = rand::thread_rng();

    // create map
    let mut maze = [[0; MAP_N]; MAP_N];
    // 外周を壁にする
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[n][MAP_N - 1] = 1;
        maze[0][n] = 1;
        maze[MAP_N - 1][n] = 1;
    }
    // 2マスに１つ壁
    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1;
            let r = rng.get_range(0..=3);
            match r {
                0 => maze[y][x - 1] = 1,
                1 => maze[y][x + 1] = 1,
                2 => maze[y - 1][x] = 1,
                3 => maze[y + 1][x] = 1,
                _ => {}
            }
        }
    }
    // output
    let tiles = ["  ", "##"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
