#[derive(Copy, Clone, PartialEq, Hash)]
enum Mark {
    O,
    X,
}

impl Mark {
    fn other(&self) -> Mark {
        match self {
            Mark::O => Mark::X,
            Mark::X => Mark::O,
        }
    }
}

#[derive(Clone, PartialEq, Hash)]
enum State {
    Turn(Mark),
    Won(Mark),
    Tie,
}

#[derive(Clone, PartialEq, Hash)]
struct Board {
    marks: [Option<Mark>; 9],
    state: State,
}

impl Board {
    fn new() -> Self {
        Board {
            marks: [None; 9],
            state: State::Turn(Mark::X),
        }
    }

    fn get_new_state(&self) -> State {
        for row in 0..3 {
            let mark = self.marks[row * 3];
            if mark.is_none() {
                continue;
            }
            if self
                .marks
                .iter()
                .skip(row * 3)
                .take(3)
                .all(|&m| m == mark)
            {
                return State::Won(mark.unwrap());
            }
        }
        for col in 0..3 {
            let mark = self.marks[col];
            if mark.is_none() {
                continue;
            }
            if self
                .marks
                .iter()
                .skip(col)
                .step_by(3)
                .take(3)
                .all(|&m| m == mark)
            {
                return State::Won(mark.unwrap());
            }
        }
        for diag in 0..2 {
            let mark = self.marks[diag * 2];
            if mark.is_none() {
                continue;
            }
            if self
                .marks
                .iter()
                .skip(diag * 2)
                .step_by(2 + diag * 2)
                .take(3)
                .all(|&m| m == mark)
            {
                return State::Won(mark.unwrap());
            }
        }

        if self.marks.iter().all(Option::is_some) {
            return State::Tie;
        }
        self.state.clone()
    }

    fn place(&mut self, index: usize) -> Option<()> {
        if index >= self.marks.len() {
            return None;
        }
        if self.marks[index].is_some() {
            return None;
        }
        match self.state {
            State::Turn(mark) => {
                self.marks[index] = Some(mark);
                self.state = State::Turn(mark.other());
                self.state = self.get_new_state();
                Some(())
            }
            _ => {
                return None;
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    while let State::Turn(_) = board.state {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: usize = match input.trim().parse() {
            Ok(u) => u,
            Err(_) => {continue;}
        };
        board.place(input);
    }

    println!("Hello, world!");
}
