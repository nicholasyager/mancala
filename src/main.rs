extern crate rand;

use rand::Rng;
use std::io;

struct Board {
    stones_per_cell: i32,
    cells: Vec<i32>
}

impl Board {
    
    fn new(stones_per_cell: i32) -> Board {
        let mut new_cells: Vec<i32> = vec![stones_per_cell; 14];
        new_cells[0] = 0;
        new_cells[7] = 0;
        Board {
            cells: new_cells,
            stones_per_cell: stones_per_cell
        }
    }

    fn completion_check(&self) -> bool {
        // Report on if a side is out of stones.
        let mut side_1 = 0;
        let mut side_2 = 0;
        for index in 1..7 {
            side_1 += self.cells[index];
        }
        if side_1 ==0 { return true}
        
        for index in 8..14 {
            side_2 += self.cells[index];
        }
        if side_2 ==0 { return true}
        return false;
    }

    fn tidy(&mut self){
        for index in 1..7 {
            self.cells[0] += self.cells[index];
            self.cells[index] = 0;
        }
        
        for index in 8..14 {
            self.cells[7] += self.cells[index];
            self.cells[index] = 0;
        }
    }


    fn distribute(&mut self, index: usize) {
        // Determine the number of stones in the cell
        let mut num_stones = self.cells[index];

        // Clear the cell
        self.cells[index] = 0;

        // Iteratively start filling up the other cells
        let mut cell_index = index+1;
        while num_stones > 0 {
            if cell_index >= 14 {
                cell_index -= 14;
            }
            self.cells[cell_index] += 1;
            num_stones -= 1;
            cell_index += 1;
        }
    }

    fn render(&self) {
        for cell in &self.cells {
            print!("{}\t", cell);
        }
        print!{"{}\t{}\n", self.cells[0], self.cells[7]};
    }

}

struct Player {
    number: usize
}


impl Player {

    fn new(player_number: usize) -> Player {
        Player {number: player_number}
    }

    fn choose_random(&self, board: &Board) -> usize {
        let mut valid_indices: Vec<usize> = Vec::new();
        let lower_bound = 1 + ((self.number - 1)*7);
        let upper_bound = lower_bound + 5;
        let mut index = lower_bound;
        while index <= upper_bound{
            if board.cells[index] > 0 {valid_indices.push(index as usize);}
            index += 1;
        }

        if valid_indices.len() == 0 {
            return lower_bound;
        }

        let random_index: usize = rand::thread_rng().gen_range(0, 
                                                        valid_indices.len());
        return valid_indices[random_index];
    }

    fn score(&self, board: &Board) -> i32 {
        let mut index = 0;
        if self.number == 2 {
            index = 7;
        }
        return board.cells[index];
    }

    fn display(&self, board: &Board) {
            println!("Player {}: {}", self.number, self.score(board));
    }
}

fn main() {

    let mut board = Board::new(3);
    let player1 = Player::new(1);
    let player2 = Player::new(2);

    board.render();
    while true {
        
        while true {
            let player1_choice = player1.choose_random(&board);
            let stones = board.cells[player1_choice];
            board.distribute(player1_choice);

            if (player1_choice as i32)+ stones != 6 {
                break;
            }
        }
        
        while true {
            let player2_choice = player2.choose_random(&board);
            let stones = board.cells[player2_choice];

            board.distribute(player2_choice);
            if (player2_choice as i32) + stones != 13 {
                break
            }

        }

        board.render();
        if board.completion_check() {
            break
        }
    }
    board.tidy();
    board.render();


}
