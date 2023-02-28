mod arg_handler;
mod generator;
mod maze_printer;

fn main() {
    let opts: arg_handler::Args = arg_handler::get_options();

    let size = generator::Pos{x: opts.width, y: opts.height};
    let map: Vec<Vec<char>> = generator::generator(size);

    maze_printer::print_maze(map);
}
