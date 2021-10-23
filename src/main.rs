use game_of_life::*;

fn main() {

    let (dimensions, initial_state): ((usize, usize), Vec<u8>) = read_csv("initial_state.csv")
                                       .unwrap();
    let rules = gol_rules();
    let config = Config::read("Config.csv").unwrap();

    let cb = ggez::ContextBuilder::new("Game of Life", "FM")
                 .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
                 .window_mode(conf::WindowMode::default().dimensions(
                         (config.pixel_size*dimensions.1) as f32, 
                         (config.pixel_size*dimensions.0) as f32));
    let (ctx, event_loop) = cb.build().unwrap();
 
    let board = Board::new(initial_state, dimensions.1);
    
    let state = MainState::new(board, rules, &config).unwrap();
    event::run(ctx, event_loop, state)
}
