use std::{char, collections::HashMap};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Vertical((u32, u32)),
    Horizontal((u32, u32)),
    NorthEastBend((u32, u32)),
    NorthWestBend((u32, u32)),
    SouthEastBend((u32, u32)),
    SouthWestBend((u32, u32)),
    Ground((u32, u32)),
    Starting((u32, u32)),
}

#[derive(Debug, Clone)]
struct PipeGrid(Vec<Vec<Pipe>>);

impl From<&str> for PipeGrid {
    fn from(value: &str) -> Self {
        let pipes = value
            .lines()
            .filter(|l| !l.trim().is_empty())
            .enumerate()
            .fold(vec![], |mut y_axis, (y, l)| {
                let y = &(y as u32);
                let line_pipes = l
                    .chars()
                    .into_iter()
                    .filter(|c| !c.is_whitespace())
                    .enumerate()
                    .fold(vec![], |mut x_axis, (x, c)| {
                        let x = &(x as u32);
                        x_axis.push(Pipe::new(&c, x, y));
                        x_axis
                    });
                y_axis.push(line_pipes);
                y_axis
            });
        PipeGrid(pipes)
    }
}

impl Pipe {
    fn new(c: &char, x: &u32, y: &u32) -> Self {
        let c = c.to_owned();
        let x = x.to_owned();
        let y = y.to_owned();
        match c {
            '|' => Self::Vertical((x, y)),
            '-' => Self::Horizontal((x, y)),
            'L' => Self::NorthEastBend((x, y)),
            'J' => Self::NorthWestBend((x, y)),
            'F' => Self::SouthEastBend((x, y)),
            '7' => Self::SouthWestBend((x, y)),
            '.' => Self::Ground((x, y)),
            'S' => Self::Starting((x, y)),
            _ => panic!("Case: {} is not covered", c),
        }
    }
    fn get_coords(&self) -> (u32, u32) {
        match self {
            Pipe::Vertical(coord) => *coord,
            Pipe::Horizontal(coord) => *coord,
            Pipe::NorthEastBend(coord) => *coord,
            Pipe::NorthWestBend(coord) => *coord,
            Pipe::SouthEastBend(coord) => *coord,
            Pipe::SouthWestBend(coord) => *coord,
            Pipe::Starting(coord) => *coord,
            Pipe::Ground(coord) => *coord,
        }
    }
    fn is_starting(&self) -> bool {
        match self {
            Pipe::Starting(_) => true,
            _ => false,
        }
    }
    fn is_ground(&self) -> bool {
        match self {
            Pipe::Ground(_) => true,
            _ => false,
        }
    }
    fn get_adjacent_pipes(&self) -> Vec<(u32, u32)> {
        let mut return_vec = vec![];
        match self {
            Pipe::Ground(_) => {}
            Pipe::Vertical((x, y)) => {
                if y != &0u32 {
                    return_vec.push((*x, y - 1))
                }
                return_vec.push((*x, y + 1))
            }
            Pipe::Horizontal((x, y)) => {
                if x != &0u32 {
                    return_vec.push((x - 1, *y))
                }
                return_vec.push((x + 1, *y))
            }
            Pipe::NorthEastBend((x, y)) => {
                return_vec.push((x + 1, *y));
                return_vec.push((*x, y - 1));
            }
            Pipe::NorthWestBend((x, y)) => {
                if x != &0u32 {
                    return_vec.push((x - 1, *y))
                }
                return_vec.push((*x, y - 1))
            }
            Pipe::SouthEastBend((x, y)) => {
                if x != &0u32 {
                    return_vec.push((*x, y + 1))
                }
                return_vec.push((x + 1, *y))
            }
            Pipe::SouthWestBend((x, y)) => {
                if x != &0u32 && x != &0u32 {
                    return_vec.push((*x, y + 1));
                    return_vec.push((x - 1, *y));
                }
            }
            Pipe::Starting((x, y)) => {
                if x != &0u32 && y != &0u32 {
                    return_vec.push((*x, y - 1));
                    return_vec.push((x - 1, *y));
                }
                return_vec.push((*x, y + 1));
                return_vec.push((x + 1, *y));
            }
        }
        return_vec
    }
}

impl PipeGrid {
    fn steps_from_starting(self) -> HashMap<(u32, u32), u32> {
        let starting_pos = self.starting_pipe().expect("No starting pipe");
        let mut map = HashMap::new();
        map.insert(starting_pos.get_coords(), 0);

        let grid = self.clone();
        // let inner_grid = self.0;

        let adj = starting_pos.get_adjacent_pipes();
        adj.iter()
            .filter(|coor| {
                if let Some(pp) = grid.pipe_at(coor) {
                    !pp.is_ground()
                } else {
                    false
                }
            })
            .for_each(|coord| {
                let adj_pipe = grid.pipe_at(coord).unwrap();
                let mut adj_adj = adj_pipe.get_adjacent_pipes();
                let mut count: u32 = 1;
                if adj_adj.contains(&starting_pos.get_coords()) {
                    map.insert(*coord, count);
                }
                loop {
                    adj_adj
                        .iter_mut()
                        .filter(|coor| {
                            if let Some(pp) = grid.pipe_at(coor) {
                                !pp.is_ground() && !pp.is_starting()
                            } else {
                                false
                            }
                        })
                        .for_each(|coo| {
                            if let Some(pp) = grid.pipe_at(coo) {
                                if pp.get_adjacent_pipes().contains(&adj_pipe.get_coords()) {
                                    count += 1;
                                    map.insert(*coo, count);
                                }
                            }
                        });
                    break;
                }
            });
        map
    }

    fn count_steps(self) -> Vec<String> {
        let grid = self.clone();
        let inner_grid = self.0;
        inner_grid.iter().fold(vec![], |mut y_axis, line| {
            println!("LINE: {:?}", line);
            let mut touched = vec![];
            let x_string = line.into_iter().fold(String::new(), |mut x_axis, pipe| {
                println!("CHECKING PIPE: {:?}", pipe);
                if !touched.contains(pipe) {
                    match pipe {
                        Pipe::Starting(_) => {
                            x_axis.push('S');
                        }
                        Pipe::Ground(_) => {
                            x_axis.push('.');
                        }
                        _ => {
                            println!();
                            let adj = pipe.get_adjacent_pipes();
                            println!("ADJACENCIES: {:?}", adj);
                            adj.iter()
                                .filter(|coor| {
                                    if let Some(pp) = grid.pipe_at(coor) {
                                        !pp.is_ground()
                                    } else {
                                        false
                                    }
                                })
                                .for_each(|coord| {
                                    let adj_pipe = grid.pipe_at(coord).unwrap();
                                    let adj_adj = adj_pipe.get_adjacent_pipes();
                                    if !touched.contains(&adj_pipe) {
                                        println!(
                                            "CHECKING ADJ PIPE: {:?}\nADJACENCIES: {:?}",
                                            adj_pipe, adj_adj
                                        );
                                        if adj_adj.contains(&pipe.get_coords()) {
                                            touched.push(*adj_pipe);
                                            println!("{:?} {:?} ARE ADJACENT", pipe, adj_pipe);
                                            x_axis.push('C');
                                        } else {
                                            println!("{:?} {:?} NOT ADJACENT", &pipe, adj_pipe);
                                        }
                                    }
                                });
                        }
                    }
                }
                println!("FINISHED X AXIS: {:?}", x_axis);
                x_axis
            });
            println!("FINISHED Y AXIS: {:?}", x_string);
            y_axis.push(x_string);
            y_axis
        })
    }
    fn pipe_at(&self, coord: &(u32, u32)) -> Option<&Pipe> {
        let (x, y) = coord;
        self.0.get(*y as usize).unwrap().get(*x as usize)
    }
    fn starting_pipe(&self) -> Option<&Pipe> {
        self.0
            .iter()
            .filter(|line| line.iter().any(|pipe| pipe.is_starting()))
            .find_map(|l| {
                l.iter()
                    .find_map(|p| if p.is_starting() { Some(p) } else { None })
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::PipeGrid;

    #[test]
    fn correctlx_parse_input_into_grid() {
        let input = ".....
            .S-7.
            .|.|.
            .L-J.
            .....";
        let grid = PipeGrid::from(input);
        grid.0.iter().for_each(|p| {
            println!("{:?}", p);
        });
        let counts = grid.steps_from_starting();
        println!("{:?}", counts);
        assert!(false)
    }
}
