mod tests;

enum Cell {
    Void,
    Amazon,
    Tree,
}

use Cell::*;

impl Cell {
    fn print(&self) {
        match self {
            Void => print!("[]"),
            Amazon => print!("A "),
            Tree => print!("T "),
        }
    }

    fn next(&self) -> Self {
        match self {
            Void => Amazon,
            Amazon => Tree,
            Tree => Void,
        }
    }
}

struct Board {
    cells: Vec<Cell>,
    side: usize,
    cell_count: usize,
}

impl Board {
    fn new(side: usize) -> Self {
        let mut vector = vec![];
        for _ in 0..side * side {
            vector.push(Cell::Void);
        }
        Self {
            cells: vector,
            side: side,
            cell_count: side * side,
        }
    }

    fn combo_count(&self) -> usize {
        return 3_i32.pow(self.cell_count as u32) as usize;
    }

    fn parse(human_string: &str) -> Self {
        let mut vector = vec![];
        for char in human_string.chars() {
            if char == ' ' {
                vector.push(Void)
            } else if char == 'T' {
                vector.push(Tree)
            } else if char == 'A' {
                vector.push(Amazon)
            }
        }

        let side = (vector.len() as f64).sqrt() as usize;

        Self {
            cells: vector,
            side: side,
            cell_count: side * side,
        }
    }

    fn amazon_count(&self) -> usize {
        println!("warning slow amazon_count");

        let mut ret = 0;

        for cell in self.cells.iter() {
            if let Amazon = cell {
                ret += 1;
            }
        }

        return ret;
    }

    fn tree_count(&self) -> usize {
        println!("warning slow tree_count");
        let mut ret = 0;

        for cell in self.cells.iter() {
            if let Tree = cell {
                ret += 1;
            }
        }

        return ret;
    }

    fn is_valid(&self) -> bool {
        for (i, cell) in self.cells.iter().enumerate() {
            if let Cell::Amazon = cell {
                if self.is_threatened(i) {
                    return false;
                }
            }
        }
        true
    }

    fn next(&mut self) {
        let mut carry = true;
        let mut i = 0;
        while carry && i < self.cell_count {
            self.cells[i] = self.cells[i].next();
            carry = if let Void = self.cells[i] {
                true
            } else {
                false
            };
            i += 1;
        }
    }

    fn print(&self) {
        println!("-----");
        for (i, cell) in self.cells.iter().enumerate() {
            cell.print();
            if i % self.side == self.side - 1 {
                println!();
            }
        }
    }

    fn is_threatened_raytrace(
        &self,
        start: usize,
        step_size: isize,
        min_elm: isize,
        max_elm: isize,
    ) -> bool {
        // println!("raytracing {start} steping {step_size} limits are {min_elm} <--> {max_elm}");

        let mut cursor = start as isize;
        cursor += step_size;
        while cursor >= min_elm && cursor <= max_elm {
            match self.cells[cursor as usize] {
                Amazon => {
                    // println!("{cursor} is threat to {start} steping {step_size} limits are {min_elm} <--> {max_elm}");
                    return true;
                }
                Tree => return false,
                Void => (),
            }
            cursor += step_size;
        }

        return false;
    }

    fn is_threatened_castle(&self, cell_index: usize) -> bool {
        let horizontal_min = (cell_index / self.side * self.side) as isize;
        let horizontal_max = ((cell_index / self.side + 1) * self.side - 1) as isize;

        self.is_threatened_raytrace(cell_index, 1, horizontal_min, horizontal_max)
            || self.is_threatened_raytrace(cell_index, -1, horizontal_min, horizontal_max)
            || self.is_threatened_raytrace(
                cell_index,
                self.side as isize,
                0,
                self.cell_count as isize - 1,
            )
            || self.is_threatened_raytrace(
                cell_index,
                -(self.side as isize),
                0,
                self.cell_count as isize - 1,
            )
    }

    fn is_threatened_bishop(&self, cell_index: usize) -> bool {
        let cell_x = cell_index % self.side;
        let cell_y = cell_index / self.side;
        let max_elm = self.cell_count as isize - 1;

        let step_pp = self.side as isize + 1;
        let cleareance_pp = (self.side - cell_x - 1).min(self.side - cell_y - 1) as isize;
        let cleareance_pp_offset = cleareance_pp * self.side as isize;
        // println!("step_pp: {step_pp} | cleareance_pp: {cleareance_pp} | cleareance_pp_offset: {cleareance_pp_offset}");

        let step_pn = self.side as isize - 1;
        let cleareance_pn = (cell_x).min(self.side - cell_y - 1) as isize;
        let cleareance_pn_offset = cleareance_pn * self.side as isize;
        // println!("step_pn: {step_pn} | cleareance_pn: {cleareance_pn} | cleareance_pn_offset: {cleareance_pn_offset}");

        let step_np = -(self.side as isize) + 1;
        let cleareance_np = (self.side - cell_x - 1).min(cell_y) as isize;
        let cleareance_np_offset = cleareance_np * self.side as isize;
        // println!("step_np: {step_np} | cleareance_np: {cleareance_np} | cleareance_np_offset: {cleareance_np_offset}");

        let step_nn = -(self.side as isize) - 1;
        let cleareance_nn = (cell_x).min(cell_y) as isize;
        let cleareance_nn_offset = cleareance_nn * self.side as isize;
        // println!("step_nn: {step_nn} | cleareance_nn: {cleareance_nn} | cleareance_nn_offset: {cleareance_nn_offset}");

        self.is_threatened_raytrace(
            cell_index,
            step_pp,
            0,
            cell_index as isize + cleareance_pp_offset,
        ) || self.is_threatened_raytrace(
            cell_index,
            step_pn,
            0,
            cell_index as isize + cleareance_pn_offset,
        ) || self.is_threatened_raytrace(
            cell_index,
            step_np,
            cell_index as isize - cleareance_np_offset,
            max_elm,
        ) || self.is_threatened_raytrace(
            cell_index,
            step_nn,
            cell_index as isize - cleareance_nn_offset,
            max_elm,
        )
    }

    fn is_threatened_horse(&self, cell_index: usize) -> bool {
        let x = (cell_index % self.side) as isize;
        let y = (cell_index / self.side) as isize;

        let offsets = [
            [1, 2],
            [1, -2],
            [-1, 2],
            [-1, -2],
            [2, 1],
            [2, -1],
            [-2, 1],
            [-2, -1],
        ];

        for [dx, dy] in offsets {
            let check_x = x + dx;
            let check_y = y + dy;
            if check_x >= 0
                && check_x < self.side as isize
                && check_y >= 0
                && check_y < self.side as isize
            {
                let check_index = check_x as usize + self.side * check_y as usize;
                if let Amazon = self.cells[check_index] {
                    return true;
                }
            }
        }

        return false;
    }

    fn is_threatened(&self, cell_index: usize) -> bool {
        self.is_threatened_castle(cell_index)
            || self.is_threatened_bishop(cell_index)
            || self.is_threatened_horse(cell_index)
    }
}

fn backtrack(
    board: &mut Board,
    cell_i: usize,
    best_tree_count: &mut usize,
    tree_count: &mut usize,
    amazon_count: &mut usize,
) {
    if *tree_count >= *best_tree_count {
        return;
    }
    if cell_i == board.cell_count {
        if *amazon_count == board.side && board.is_valid() {
            board.print();
            println!("prev_tree_count: {}", best_tree_count);
            *best_tree_count = *tree_count;

            println!("best_tree_count: {}", best_tree_count);
        }
    } else {
        if *amazon_count < board.side {
            *amazon_count += 1;
            board.cells[cell_i] = Amazon;
            if !board.is_threatened(cell_i) {
                backtrack(board, cell_i + 1, best_tree_count, tree_count, amazon_count);
            }
            *amazon_count -= 1;
        }
        *tree_count += 1;
            board.cells[cell_i] = Tree;
            backtrack(board, cell_i + 1, best_tree_count, tree_count, amazon_count);
        *tree_count -= 1;

        board.cells[cell_i] = Void;
            backtrack(board, cell_i + 1, best_tree_count, tree_count, amazon_count);
        
    }
}

fn main() {
    let side = 8;
    let mut base_board = Board::new(side);
    let mut best_tree_count = side*side; // shotcut
    backtrack(&mut base_board, 0, &mut best_tree_count, &mut 0, &mut 0);
}
