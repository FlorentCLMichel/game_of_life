//! A simple implementation of Conway's game of life in two dimensions with periodic boundary 
//! conditions

pub struct Board {
    dimensions: (usize, usize),
    states: Vec<u8> // the state of each cell is determined by a u8: 
                    // 1: alive
                    // 0: dead
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
            states
        }
    }

    pub fn evolve(&mut self, rules: &Rules) {
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
                new_states.push(
                    if ((self.states[i*ny+j] == 1) & rules.stay_alive.contains(&n_alive_neignbours)) 
                        | ((self.states[i*ny+j] == 0) & rules.come_to_life.contains(&n_alive_neignbours)) 
                    { 1 } else { 0 });
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
