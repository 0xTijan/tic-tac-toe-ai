use crate::check_for_winners;
use crate::find_empty_fields;
use crate::random_bot_move;
use crate::FieldData;
use crate::Board;
use rand::Rng;
use std::time::{Instant, Duration};
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, Clone, PartialEq)]
struct State {
    board: Board,
    board_status: FieldData,
    current_player: FieldData,
    visits: i64,
    win_score: i64,
}

impl State {
    fn new(board: Board, current_player: FieldData, visits: Option<i64>, win_score: Option<i64>, board_status: Option<FieldData>) -> State {
        State {
            board,
            current_player,
            visits: visits.unwrap_or(0),
            win_score: win_score.unwrap_or(0),
            board_status: board_status.unwrap_or(FieldData::None)
        }
    }

    fn get_all_possible_states(&self) -> Vec<State> {
        let empty_fields = find_empty_fields(&self.board);
        let mut states: Vec<State> = Vec::new();
        
        for i in empty_fields {
            let mut board = self.board.clone();
            board[i / 3][i % 3] = FieldData::get_opponent(&self.current_player);

            states.push(State {
                board: board,
                current_player: FieldData::get_opponent(&self.current_player),
                visits: 0,
                win_score: 0,
                board_status: FieldData::None
            });
        }

        states
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    fn find_best_ucb_child(&self) -> Rc<RefCell<Node>> {
        let parent_visit_count = self.state.visits;

        // loop though all child nodes
        // return the node with the best ucb score
        self.children
            .iter()
            .max_by(|child_a, child_b| {
                let child_a_ucb = {
                    let child_a_borrow = child_a.borrow();
                    calculate_ucb(&parent_visit_count, &child_a_borrow.state.win_score, &child_a_borrow.state.visits)
                };

                let child_b_ucb = {
                    let child_b_borrow = child_b.borrow();
                    calculate_ucb(&parent_visit_count, &child_b_borrow.state.win_score, &child_b_borrow.state.visits)
                };

                // compare ucb values
                child_a_ucb.partial_cmp(&child_b_ucb).unwrap()
            })
            .unwrap()
            .clone() // return the child with the highest UCB score
    }

    fn get_random_child(&self) -> Option<Rc<RefCell<Node>>> {
        // Check if the node has any children
        if self.children.is_empty() {
            None
        } else {
            // Generate a random index
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..self.children.len());
            // Return the random child
            Some(Rc::clone(&self.children[index]))
        }
    }
}

#[derive(Debug, Clone)]
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

    fn get_root_node(&self) -> Node {
        self.root.borrow().clone()
    }
}

fn calculate_ucb(parent_visits: &i64, win_score: &i64, node_visits: &i64) -> f64 {
    if *parent_visits == 0 {
        f64::MAX
    } else {
        // calculate UCB using UCB formula 
        (*win_score as f64 / *node_visits as f64) + 1.41 * f64::sqrt(f64::ln(*parent_visits as f64) / *node_visits as f64)
    }
}

fn add_score(state: &mut State, score: i64) -> State {
    if state.win_score != i64::MIN {
        state.win_score += score;
    }
    return state.clone();
}

// monte carlo tree search main function
pub fn mcts(board: &Board, player: FieldData, duration_sec: u64) -> usize {
    let opponent = FieldData::get_opponent(&player);

    let state = State::new(*board, player, None, None, None);
    let mut tree = Tree::new(state);

    println!("tree {:?}", tree);

    let start_time = Instant::now();   // Get the current time (start time)
    let duration = Duration::new(duration_sec, 0);  // Set the duration for duration_sec seconds
    
    // run MCTS algorithm (repeating all 4 phases) for allowed time
    while Instant::now() - start_time < duration {
        // 1. SELECTION PHASE
        let mut selected_node = select_node(tree.get_root_node());
        
        // 2. EXPANSION PHASE
        selected_node = expand_node(selected_node);

        if let Some(n) = selected_node.get_random_child() {
            selected_node = n.borrow().clone();
        }

        // 3. SIMULATION PHASE
        let simulation_result = simulate_random_play(&selected_node, &opponent);
        
        // 4. BACK-PROPAGATION PHASE
        selected_node = back_propagation(&mut selected_node, &simulation_result);

        // update tree
        tree.root = Rc::new(RefCell::new(selected_node));
    }

    println!("Done with while loop! {:#?}", tree.get_root_node());
    let winner_node = tree.get_root_node().find_best_ucb_child().borrow().clone();
    println!("Winner node {:?}", winner_node);
    1
}

// SELECTION PHASE
fn select_node(root_node: Node) -> Node {
    if root_node.children.len() == 0 {
        root_node
    } else {
        let res = root_node.find_best_ucb_child();
        let a = res.borrow();
        return a.clone();   
    }
}

// EXPANSION PHASE
fn expand_node(mut node: Node) -> Node {
    let possible_states = node.state.get_all_possible_states();
    let parent_node = Rc::new(RefCell::new(node.clone()));  // Clone node here to keep a copy

    for state in possible_states {
        // add it to the child nodes of the parent node
        node.add_child(state.clone(), parent_node.clone());
    }

    return node;
}

// SIMULATION PHASE
fn simulate_random_play(node: &Node, opponent: &FieldData) -> FieldData {
    let mut board = node.state.board.clone();
    let mut empty_fields = find_empty_fields(&board);
    let mut curr_player = opponent;

    let mut game_status = check_for_winners(&board);
    if game_status == *opponent {
        return game_status;
    }

    while game_status == FieldData::None && empty_fields.len() > 0{
        // update the current player for the next move
        curr_player = match curr_player {
            FieldData::O => &FieldData::X,
            _ => &FieldData::O,
        };

        // update board - random play
        let field_num = random_bot_move(&board);
        board[field_num / 3][field_num % 3] = *curr_player;

        // update empty fields
        empty_fields = find_empty_fields(&board);

        // update game status
        game_status = check_for_winners(&board);
    }

    return game_status;
}

// BACK PROPAGATION
fn back_propagation(node: &mut Node, simulation_result: &FieldData) -> Node {
    let mut temp_node = Some(node.clone());

    while temp_node != None {
        node.state.visits += 1;
        if node.state.board_status == *simulation_result {
            node.state = add_score(&mut node.state, 10);
        }
        //! MUST UPDATE ALSO THE NODE PARENT but with the data updated in the
        temp_node = temp_node.unwrap().parent.as_ref().map(|rc| rc.borrow().clone());
    }

    return node.clone();
}