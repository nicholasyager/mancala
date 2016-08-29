extern crate rand;

use rand::Rng;

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

const USAGE: &'static str = "
Mancala Simulator

Usage:
  mancala <strategy1> <strategy2> [--stones=<num>]

Options:
  -h --help           Show this screen.
  strategy1           The strategy player one will use.
  strategy2           The strategy player two will use.
  --stones=<num>      The number of stones per cell [default: 3]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_stones: i32,
    arg_strategy1: String,
    arg_strategy2: String,
}

struct Board {
    cells: Vec<i32>
}

impl Board {
    
    fn new(stones_per_cell: i32) -> Board {
        let mut new_cells: Vec<i32> = vec![stones_per_cell; 14];
        new_cells[0] = 0;
        new_cells[7] = 0;
        Board {
            cells: new_cells,
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
    number: usize,
	strategy: String
}


impl Player {

    fn new(player_number: usize, strategy: String) -> Player {
        Player {number: player_number, strategy: strategy}
    }

	fn choose(&self, board: &Board) -> usize {
		if self.strategy == "points" {
			// Favor giving yourself points
			return self.choose_random(board);
			
		} else if self.strategy == "theft" {
			// Favor stealing your opponent's stones
			return self.choose_random(board);
	
		} else if self.strategy == "optimal" {
			// Maximize points from the two above.
			return self.choose_random(board);

		} else {
			// Randomly select a cell to distribute.
			return self.choose_random(board);

		}
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

    fn play(&self, board: &mut Board) {
        loop {
            let player1_choice = self.choose(&board);
            let stones = board.cells[player1_choice];
            board.distribute(player1_choice);

            if (player1_choice as i32)+ stones != ((self.number as i32)-1)*7 {
                return;
            }
        }

    }
}

fn main() {

	let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

	let stones = args.flag_stones;
	println!("{}", stones);

    let mut board = Board::new(stones);
    let player1 = Player::new(1, args.arg_strategy1);
    let player2 = Player::new(2, args.arg_strategy2);

    board.render();
    loop {
        player1.play(&mut board);
        player2.play(&mut board);

        board.render();
        if board.completion_check() {
            break
        }
    }
    board.tidy();
    board.render();


}
