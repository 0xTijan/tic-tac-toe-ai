use crate::FieldData;
use crate::Board;
use crate::check_for_winners;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::rc::Rc;
use std::cell::RefCell;


struct State {
    board: Board,
    current_player: FieldData,
    visits: i64,
    win_score: i64,
}

impl State {
    fn new(board: Board, current_player: FieldData, visits: Option<i64>, win_score: Option<i64>) -> State {
        State {
            board,
            current_player,
            visits: visits.unwrap_or(0),
            win_score: win_score.unwrap_or(0),
        }
    }
}

struct Node {
    state: State,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(state: State, parent: Option<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            state,
            parent,
            children: Vec::new(),
        }))
    }
    
    fn add_child(&mut self, child_state: State, parent: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let child = Node::new(child_state, Some(parent.clone()));
        self.children.push(child.clone());  // Add child to the list of children
        child
    }
}

struct Tree {
    root: Rc<RefCell<Node>>,
}

impl Tree {
    fn new(state: State) -> Tree {
        let root_node = Node::new(state, None);
        Tree {
            root: root_node,
        }
    }
}


// monte carlo tree search main function
pub fn mcts(board: &Board, player: FieldData, duration_sec: u64) {
    let opponent = FieldData::get_opponent(&player);

    let state = State::new(*board, player, None, None);
    let tree = Tree::new(state);

    let start_time = Instant::now();   // Get the current time (start time)
    let duration = Duration::new(duration_sec, 0);  // Set the duration for duration_sec seconds
    
    // run MCTS algorithm (repeating all 4 phases) for allowed time
    while Instant::now() - start_time < duration {
        // 1. SELECTION PHASE
        // 2. EXPANSION PHASE
        // 3. SIMULATION PHASE
        // 4. BACK-PROPAGATION PHASE
    }
}

