fn main() {
    let mut grid = Grid::create(10, 10);
    grid.print();

    /* Looping Code */
    let loops: i8 = 16;
    for _ in 0..loops{
        grid.print();
        grid.update();
        println!()
    }
}

struct Grid{
    x: usize,
    y: usize,
    grid: Vec<Vec<i64>>
}

trait GridM {
    fn print(&self);
    fn create(x: usize, y:usize) -> Grid;
    fn update(&mut self);
    fn neighbours(&self, x: usize, y: usize) -> i64;
}


impl GridM for Grid {
    fn print(&self) {
        for row in &self.grid {
            for &val in row {
                print!("{}", if val == 1 { '*' } else { '.' });
            }
            println!()
        }
    }

    fn create(x: usize, y: usize) -> Self {
        Grid { 
            x, 
            y, 
            grid: vec![
                vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0], /* Glider pattern */
                vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ]
        }
    }

    fn update(&mut self){
        let mut new_grid: Vec<Vec<i64>> = vec![vec![0; self.y]; self.x];

        for i in 0..self.x {
            for j in 0..self.y {
                let live_neighbors = self.neighbours(i, j);
                if self.grid[i][j] == 1 {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        new_grid[i][j] = 0; 
                    } else {
                        new_grid[i][j] = 1;
                    }
                } else {
                    if live_neighbors == 3 {
                        new_grid[i][j] = 1;
                    }
                }
            }
        }

        self.grid = new_grid;
    }

    fn neighbours(&self, x: usize, y: usize) -> i64 {
        let mut count = 0;

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), /*(0, 0),*/ (0, 1),
            (1, -1), (1, 0), (1, 1)
        ]; /* Eight for the eight directions */

        for (d_x, d_y) in directions{
            let new_x = x as isize + d_x;
            let new_y = y as isize + d_y;

            if new_x >= 0 && new_x < self.x as isize && new_y >= 0 && new_y < self.y as isize {
                count += self.grid[new_x as usize][new_y as usize];
            }
        }

        count
    }
}

