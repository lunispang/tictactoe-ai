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

    fn to_char(&self) -> char {
        match self {
            Self::O => 'O',
            Self::X => 'X',
        }
    }

    fn to_value(&self) -> i8 {
        match self {
            Self::O => -1,
            Self::X => 1,
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
            marks: [None; 9], state: State::Turn(Mark::X), }
    }

    fn print(&self) {
        for i in 0..9 {
            print!("{}", match self.marks[i as usize] {
                None => (b'0' + i) as char,
                Some(m) => m.to_char(),
            });
            if i%3 == 2 {
                println!();
            } else {
                print!("|");
            }
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
    
    fn is_full(&self) -> bool { self.marks.iter().all(|m| m.is_some()) }
}

#[derive(Clone)]
enum NodeType {
    Unfinished(Board),
    Value(i8),
}

#[derive(Clone)]
struct MiniMaxNode {
    kind: NodeType,
    moves: Vec<u8>,
}

impl MiniMaxNode {
    fn new(board: &Board) -> Self {
        Self {
            kind: NodeType::Unfinished(board.clone()),
            moves: Vec::new(),
        }
    }
    fn calculate(self) -> u8 {
        let res = minimax(self);
        return *res.moves.last().unwrap();
    }
}

fn minimax(node: MiniMaxNode) -> MiniMaxNode {
    match node.kind {
        NodeType::Unfinished(board) => {
            let state = board.get_new_state();
            match state {
                State::Won(m) => MiniMaxNode {moves: node.moves, kind: NodeType::Value(m.to_value())},
                State::Turn(m) => {
                    let possible: Vec<u8> = board.marks.iter().enumerate().filter(|(_, &x)| x.is_none()).map(|i| i.0 as u8).collect();
                    let mut results: Vec<MiniMaxNode> = Vec::new();
                    for mve in possible {
                        let mut new_board = board.clone();
                        new_board.place(mve as usize).unwrap();
                        results.push(
                            MiniMaxNode {
                                moves: {
                                    let mut new = node.moves.clone();
                                    new.push(mve);
                                    new
                                },
                                kind: NodeType::Unfinished(new_board)
                            }
                        )
                    }
                    let mut results: Vec<MiniMaxNode> = results.into_iter().map(|r| minimax(r)).collect();
                    results.sort_by_key(|n| {
                        match n.kind {
                            NodeType::Unfinished(_) => panic!("either memory or the rules of tic tac toe are broken"),
                            NodeType::Value(i) => i * m.to_value(),
                        }
                    });
                    return results[0].clone();
                },
                State::Tie => MiniMaxNode {moves: node.moves, kind: NodeType::Value(0)}
            }
        }
        _ => todo!(),
    }
}

//enum PlayerType {
//    Stdin,
//    MiniMax,
//}

fn main() {
    let mut board = Board::new();
    while let State::Turn(_) = board.state {
        board.print();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: usize = match input.trim().parse() {
            Ok(u) => u,
            Err(_) => {continue;}
        };
        if board.place(input).is_none() {
            println!("Invalid move.");
        }
        board.print();
        if !board.is_full() {
            let mve = MiniMaxNode::new(&board);
            board.place(mve.calculate() as usize);
        }
    }

    match board.state {
        State::Turn(_) => panic!("What the fuck"),
        State::Won(m) => println!("{} Won!", m.to_char()),
        State::Tie => println!("Tie!"),
    }
}
