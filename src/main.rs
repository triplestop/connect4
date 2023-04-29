extern crate simple_error;
extern crate text_io;

const WIN_CONDITION: usize = 4;
const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const NUM_PLAYERS: usize = 2;
const DEFAULT_PLAYERS: [char; 2]  = ['A', 'B'];

#[derive(PartialEq)]
enum State {
    Playing,
    Ended(String),
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

type D = Direction;

impl Direction {
    pub fn transform(&self, coord: &[i32; 2]) -> [i32; 2] {
        let increment: [i32; 2] = match self {
            D::Up => [0, 1],
            D::UpRight => [1, 1],
            D::Right => [1, 0],
            D::DownRight => [1, -1],
            D::Down => [0, -1],
            D::DownLeft => [-1, -1],
            D::Left => [-1, 0],
            D::UpLeft =>[-1, 1],
        };
        [coord[0] + increment[0], coord[1] + increment[1]]
    }
}

struct Game {
    board: [[char; HEIGHT]; WIDTH],
    players: [char; NUM_PLAYERS],
    curr_player: usize,
    move_count: usize,
    status: State,
}

impl Game {
    pub fn new() -> Self {
        return Self{
            board: [[' '; HEIGHT]; WIDTH],
            players: DEFAULT_PLAYERS,
            curr_player: 0,
            move_count: 0,
            status: State::Playing,
        };
    }

    pub fn play(&mut self) {
        while !matches!(self.status, State::Ended(_)) {
            self.print();
            let mut result: Result<_, _> = Err(simple_error::simple_error!(""));
            while result.is_err() {
                println!("{}", result.as_ref().expect_err("result must be error"));
                print!("Player {}, please input your next move: ", self.players[self.curr_player]);
                let input: Result<usize, _> = text_io::try_read!();
                result = match input {
                    Ok(v) => match (0..WIDTH).contains(&(v as usize)) {
                        true => self.try_drop(v),
                        _ => Err(simple_error::simple_error!("input {} not a valid move", v)),
                    },
                    Err(e) => Err(simple_error::simple_error!("{}", e)),
                }
            }
            println!();
        }
        self.print();

        let State::Ended(msg) = &self.status else { panic!() };
        println!("{}", msg);
    }

    fn try_drop(&mut self, c: usize) -> simple_error::SimpleResult<()> {
        for r in 0..HEIGHT {
            if self.board[c][r] != ' ' {
                continue;
            }
            self.board[c][r] = self.players[self.curr_player];
            if self.check_win(&[c as i32, r as i32]) {
                self.status = State::Ended(format!("Game over, player {} wins!", self.players[self.curr_player]));
            } else {
                self.move_count += 1;
                if self.move_count == HEIGHT * WIDTH {
                    self.status = State::Ended(String::from("Game over, draw!"));
                    return Ok(());
                }
                self.curr_player = self.move_count % 2;
            }
            return Ok(());
        }
        simple_error::bail!("column {} full, choose a different column", c);
    }

    #[inline(always)]
    fn check_win(&self, coord: &[i32; 2]) -> bool {
        self.count_consecutive(coord, D::Up) + self.count_consecutive(coord, D::Down) > WIN_CONDITION ||
        self.count_consecutive(coord, D::UpRight) + self.count_consecutive(coord, D::DownLeft) > WIN_CONDITION ||
        self.count_consecutive(coord, D::Right) + self.count_consecutive(coord, D::Left) > WIN_CONDITION ||
        self.count_consecutive(coord, D::DownRight) + self.count_consecutive(coord, D::UpLeft) > WIN_CONDITION
    }

    fn count_consecutive(&self, curr: &[i32; 2], direction: Direction) -> usize {
        let next_coord = direction.transform(curr);
        if next_coord[0] < 0 || next_coord[0] >= WIDTH as i32 || next_coord[1] < 0 || next_coord[1] >= HEIGHT as i32 ||
            self.board[next_coord[0] as usize][next_coord[1] as usize] != self.players[self.curr_player] {
                return 1;
        }
        1 + self.count_consecutive(&next_coord, direction)
    }

    fn print(&self) {
        for r in (0..HEIGHT).rev() {
            print!("[ ");
            for c in 0..WIDTH {
                print!("{} ", self.board[c][r]);
            }
            println!("]");
        }
        print!("  ");
        for r in 0..WIDTH {
            print!("{} ", r);
        }
        println!(" \n");
    }
}

fn main() {
    Game::new().play();
}

