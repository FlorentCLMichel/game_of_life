//! A simple implementation of Conway's game of life in two dimensions with periodic boundary 
//! conditions and random flip

use rand::{ Rng, RngCore };

mod graph;
pub use graph::*;

pub struct Config {
    pub wait_time_micros: u64,
    pub init_wait_time_micros: u64,
    pub pixel_size: usize,
    pub dimensions: (usize, usize),
    pub col1: (f32,f32,f32),
    pub col2: (f32,f32,f32),
    pub flip_proba: (usize, usize)
}

impl Config {
    pub fn read(fname: &str) -> Result<Config, Box<dyn std::error::Error>> {

        let err_message = "Missing argument in the config file";
    
        // load the content
        let content = std::fs::read_to_string(fname)?;

        // separate the nubers
        let mut content = content.split(' ');
    
        // read the content
        Ok(Config {
            wait_time_micros: content.next().ok_or(err_message)?.parse::<u64>()?,
            init_wait_time_micros: content.next().ok_or(err_message)?.parse::<u64>()?,
            pixel_size: content.next().ok_or(err_message)?.parse::<usize>()?,
            dimensions : (
                content.next().ok_or(err_message)?.parse::<usize>()?,
                content.next().ok_or(err_message)?.parse::<usize>()?),
            col1 : (
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?),
            col2 : (
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?,
                content.next().ok_or(err_message)?.parse::<f32>()?),
            flip_proba : (
                content.next().ok_or(err_message)?.parse::<usize>()?,
                content.next().ok_or(err_message)?.parse::<usize>()?)
        })

    }
}

pub struct Board {
    pub dimensions: (usize, usize),
    pub states: Vec<u8>, // the state of each cell is determined by a u8: 
                         // 1: alive
                         // 0: dead
    rng: Box<dyn RngCore>
}

pub struct Rules {
    stay_alive: Vec<u8>,
    come_to_life: Vec<u8>
}

pub fn gol_rules() -> Rules {
    Rules { stay_alive: vec![2,3], come_to_life: vec![3] }
}

impl Board {

    pub fn new(states: Vec<u8>, n_cols: usize) -> Board {
        Board {
            dimensions: (states.len() / n_cols, n_cols),
            states,
            rng: Box::new(rand::thread_rng())
        }
    }

    pub fn update(&mut self, rules: &Rules, flip_proba: (usize, usize)) {
        let (nx, ny) = self.dimensions;
        let mut new_states = Vec::<u8>::new();
        for i in 0..nx {
            let im = if i == 0 { nx-1 } else { i-1 };
            let ip = if i == nx-1 { 0 } else { i+1 };
            for j in 0..ny {
                let jm = if j == 0 { ny-1 } else { j-1 };
                let jp = if j == ny-1 { 0 } else { j+1 };

                // count the number of alive neighbours
                let n_alive_neignbours = self.states[im*ny+jm]
                                         + self.states[im*ny+j]
                                         + self.states[im*ny+jp]
                                         + self.states[i*ny+jm]
                                         + self.states[i*ny+jp]
                                         + self.states[ip*ny+jm]
                                         + self.states[ip*ny+j]
                                         + self.states[ip*ny+jp];

                // check if the cell is alive at the next step
                let mut state = 
                    if ((self.states[i*ny+j] == 1) & rules.stay_alive.contains(&n_alive_neignbours)) 
                        | ((self.states[i*ny+j] == 0) & rules.come_to_life.contains(&n_alive_neignbours)) 
                    { 1 } else { 0 };

                // random flip
                if self.rng.gen_range(0..flip_proba.1) < flip_proba.0 {
                    state = 1 - state;
                }

                new_states.push(state);
            }
        }
        self.states = new_states;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (nx, ny) = self.dimensions;
        for i in 0..nx {
            for j in 0..ny {
                if self.states[i*ny+j] == 1 {
                    write!(f, "█")?;
                } else {
                    write!(f, "░")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Rules {
    pub fn new(stay_alive: Vec<u8>, come_to_life: Vec<u8>) -> Rules {
        Rules { stay_alive, come_to_life }
    }
}
    

/// read a state file, space- and newline-separated
///
/// The file must contain only 0s and 1s and all rows need to have the same length.
pub fn read_csv(fname: &str) -> Result<((usize, usize), Vec<u8>), Box<dyn std::error::Error>> {
    
    // load the content
    let content = std::fs::read_to_string(fname)?;

    // separate in rows
    let content = content.split('\n').collect::<Vec<&str>>();
    let n_rows = content.len() - 1;

    // number of columns
    let n_cols = content[0].split(' ').collect::<Vec<&str>>().len() - 1;

    // load the data
    let mut data = Vec::<u8>::new();
    for row in content {
        for el in row.split(' ') {
            if let Ok(x) = el.parse::<u8>() { data.push(x) };
        }
    }
   
    Ok(((n_rows, n_cols), data))
}
