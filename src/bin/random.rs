use rand::Rng;
use game_of_life::*;

fn main() {

    let max_int_rng = 100;
    let threshold_alive = 80;
    let rules = gol_rules();
    let config = Config::read("Config.csv").unwrap();

    let cb = ggez::ContextBuilder::new("collision", "FM")
                 .window_setup(conf::WindowSetup::default().title("Conway's Game of Life"))
                 .window_mode(conf::WindowMode::default().dimensions(
                         (config.pixel_size*config.dimensions.1) as f32, 
                         (config.pixel_size*config.dimensions.0) as f32));
    let (ctx, event_loop) = cb.build().unwrap();

    // generate the initial state
    let mut rng = rand::thread_rng();
    let initial_state: Vec<u8> = (0..(config.dimensions.0*config.dimensions.1))
                                     .map(|_| if rng.gen_range(0..=max_int_rng) > threshold_alive
                                          { 1 } else { 0 }).collect(); 
    
    
    let board = Board::new(initial_state, config.dimensions.1);
    
    let state = MainState::new(board, rules, &config).unwrap();
    event::run(ctx, event_loop, state)
}
