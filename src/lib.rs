use std::fmt;

// 将一个平面的细胞群抽象成一维的
#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

// 整个图像由长宽和细胞组成
#[derive(PartialEq, Clone)]
pub struct Universe {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Cell>,
}

impl Cell {
    // 利用随机数随机生成细胞状态
    fn init_cells(width: i32, height: i32) -> Vec<Cell> {
        let mut cells = Vec::new();
        for i in 0..(width * height) {
            if i % 5 == 0 {
                cells.push(Cell::Alive);
            } else {
                cells.push(Cell::Dead);
            }
        }

        cells
    }
}

impl Universe {
    // 利用随机生成的细胞生成一个随机图像
    pub fn init(x: i32, y: i32) -> Universe {
        let first_cells = Cell::init_cells(x, y);
        Universe {
            width: x,
            height: y,
            cells: first_cells,
        }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        let result = y * self.width + x;
        result as usize
    }

    fn alive_neighbor_count(&self, x: i32, y: i32) -> i32 {
        let mut num: i32 = 0;

        // ngb_x&y 是相邻格子的坐标
        for ngb_x in [x - 1, x, x + 1].iter() {
            for ngb_y in [y - 1, y, y + 1].iter() {
                if ngb_x == &x && ngb_y == &y {
                    continue;
                }

                // 通过这里的处理可以让边框的格子找到足够的邻居(最左边的会补充到最右边,其他情况同理)
                let ngb_x = (ngb_x + self.width) % self.width;
                let ngb_y = (ngb_y + self.height) % self.height;

                let index = self.get_index(ngb_x, ngb_y);

                // print!("({}, {})", ngb_x, ngb_y);

                match self.cells[index] {
                    Cell::Alive => {
                        // print!("add!");
                        num += 1;
                    }
                    Cell::Dead => (),
                }
            }
        }
        // println!("  num = {};", num);

        num
    }

    // 让指定位置的细胞更改为存活
    pub fn fresh_cell_alive(&mut self, x: i32, y: i32) {
        let index = self.get_index(x, y);
        self.cells[index] = Cell::Alive;
    }

    // 让指定位置的细胞更改为死亡
    pub fn fresh_cell_dead(&mut self, x: i32, y: i32) {
        let index = self.get_index(x, y);
        self.cells[index] = Cell::Dead;
    }

    // 下一帧的图像
    pub fn next_frame(&mut self) {
        let mut new_cells = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let count = self.alive_neighbor_count(x, y);
                // print!("({}, {}): {}", x, y, count);
                let index = self.get_index(x, y);

                if self.cells[index] == Cell::Alive {
                    // 对于存活的细胞 2 | 3 个存活的邻居使得这个细胞在下一帧存活
                    match count {
                        2 | 3 => {
                            // println!("->1,");
                            new_cells.push(Cell::Alive)
                        }
                        _ => {
                            // println!("->0,");
                            new_cells.push(Cell::Dead)
                        }
                    }
                } else {
                    // 对于死亡的细胞,3个存活的邻居使得这个细胞在下一帧复活
                    match count {
                        3 => {
                            // println!("->1,");
                            new_cells.push(Cell::Alive)
                        }
                        _ => {
                            // println!("->0,");
                            new_cells.push(Cell::Dead)
                        }
                    }
                }
            }
        }
        println!();

        self.cells = new_cells;
    }
}

// 自定义的显示规则
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { "  " } else { "██" };
                // let symbol = if cell == Cell::Dead { "◻ " } else { "◼ " };

                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
