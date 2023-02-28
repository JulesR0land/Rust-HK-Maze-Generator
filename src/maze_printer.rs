

pub fn print_maze(map: Vec<Vec<char>>) {
    for row in map {
        for c in row {
            if c == '.' {
                print!(" ");
            } else {
                print!("{}", c);
            }
        }
        print!("\n");
    }
}