use crate::check_for_winners;
use crate::find_empty_fields;
use crate::random_bot_move;
use crate::FieldData;
use crate::Board;


const C: f32 = 1.414;         // sqrt(2)
const LIMIT: i32 = 100_000;   // change to higher value later

#[derive(Clone)]
struct Node {
  parent: Option<usize>,      // id of parent node (None if root node)
  children: Vec<usize>,       // list ids of children nodes
  value: i32,                 // v(i)
  visits: i32,                // n(i)
  state: Board,               // s(i)
  action: Option<usize>,      // move that vas played
  id: usize,                  // index in tree.nodes
}

struct Tree {
  nodes: Vec<Node>,           // list of all nodes
  root: usize,                // id of root node  
}

impl Node {
  pub fn new(parent: Option<usize>, state: Board, id: usize, action: Option<usize>) -> Node {
    Node {
      parent,
      children: Vec::new(),
      value: 0,
      visits: 0,
      state,
      action,
      id,
    }
  }
}

impl Tree {
  pub fn initialize(root: Node) -> Tree {
    Tree {
      nodes: vec![root],
      root: 0,
    }
  }

  pub fn insert_new_node(&mut self, node: Node) {
    self.nodes.push(node.clone());
    let new_node_index = self.nodes.len()-1;
    let parent_index = match node.parent {
      None => 0,
      Some(x) => x,
    };
    self.nodes[parent_index].children.push(new_node_index);
  }

  pub fn back_propagate(&mut self, mut node_index: usize, value: i32) {
    // increases visits += 1 and value += value for each node to the root
    while let Some(node) = self.nodes.get_mut(node_index) {
      node.visits += 1;
      node.value += value;
      if let Some(parent_index) = node.parent {
        node_index = parent_index;
      } else {
        break;
      }
    }
  }

  pub fn get_best_action(&self, node_index: usize) -> usize {
    // return action of node out of node_index's children with best value
    let node_children = &self.nodes[node_index].children;
    let mut max_value = i32::MIN;
    let mut index_of_max = node_children[0];

    for i in node_children {
      let _value = self.nodes[*i].value;
      if _value > max_value {
        index_of_max = *i;
        max_value = _value;
      }
    }

    return match self.nodes[index_of_max].action {
      Some(x) => x,
      None => 0
    };
  }
}


// MAIN FUNCTION
pub fn mcts(board: &mut Board, first_player: &FieldData) -> usize {
  // setup tree and initial nodes
  let mut tree = Tree::initialize(Node::new(None, board.clone(), 0, None));
  tree.nodes[tree.root].visits += 1;

  // MCTS main loop - run until repetition limit
  while tree.nodes[tree.root].visits <= LIMIT { 
    // 1. SELECTION PHASE 
    let selected_index = selection(&tree);

    if tree.nodes[selected_index].visits == 0 {
      // 3. ROLLOUT - no node expansion
      let rollout_value = rollout(selected_index, &mut tree, &first_player);
      // 4. BACK PROPAGATION
      tree.back_propagate(selected_index, rollout_value);
    } else {
      // 2. NODE EXPANSION
      let selected_new_node_index = node_expansion(selected_index, &mut tree, &first_player);
      // 3. ROLLOUT
      let rollout_value = rollout(selected_new_node_index, &mut tree, &first_player);
      // 4. BACK PROPAGATION
      tree.back_propagate(selected_new_node_index, rollout_value);
    }
  }

  tree.get_best_action(tree.root)
}

// 1. SELECTION
fn selection(tree: &Tree) -> usize {
  _selection(tree.root, tree)
}

fn _selection(current_index: usize, tree: &Tree) -> usize {
  let current_node = &tree.nodes[current_index];

  // return current_node if it has no children - leaf node
  if current_node.children.len() == 0 {
    return current_index;
  }
  
  let mut max_ucb: f32 = f32::MIN;
  let mut index_of_max: usize = current_node.children[0];
  
  // calculate UCB score for each of the children nodes
  for i in &current_node.children {
    if max_ucb == f32::INFINITY {
      break;
    }
    let ucb = calculate_ucb(*i, &tree);
    if ucb > max_ucb {
      max_ucb = ucb;
      index_of_max = *i;
    }
  }
  
  // recursively call this function with the node with the highest UCB as current node
  return _selection(index_of_max, tree);
}


// 3. ROLLOUT or simulation
fn rollout(node_index: usize, tree: &mut Tree, first_player: &FieldData) -> i32 {
  // get necessary data (state, parent id and empty fields)
  let state = {
    let node = &tree.nodes[node_index];
    node.state.clone()
  };
  let empty_fields = find_empty_fields(&state).len();
  let mut new_state = state.clone();

  // simulate random moves
  for _i in 0..empty_fields {
    if check_for_winners(&new_state) != FieldData::None {
      // stop rollout if game has ended
      break;
    }
    let player = check_who_plays(&new_state, first_player);
    let bot_move = generate_optimal_move(&new_state, &player);    // both players play optimally
    new_state[bot_move / 3][bot_move % 3] = player;
  }

  // return value (result of simulation)
  return match check_for_winners(&new_state) {
    FieldData::None => 0,   // draw
    FieldData::O => 1,      // algo won
    FieldData::X => -2,     // human won
  };
}


// 3. NODE EXPANSION
fn node_expansion(node_index: usize, tree: &mut Tree, first_player: &FieldData) -> usize {
  // get necessary data (state, parent id and empty fields)
  let (parent_id, state) = {
    let node = &tree.nodes[node_index];
    (node.id, node.state.clone())
  };
  let empty_fields = find_empty_fields(&state);
  let mut children_ids: Vec<usize> = Vec::new();
  
  // for each empty field add a new node
  for empty_index in empty_fields {
    let mut new_state = state.clone();
    new_state[empty_index / 3][empty_index % 3] = check_who_plays(&state, first_player);
    let child_id = tree.nodes.len();
    let new_node = Node::new(Some(parent_id), new_state, child_id, Some(empty_index));
    children_ids.push(child_id);
    tree.insert_new_node(new_node);
  }

  // return random child id
  *children_ids.first().unwrap_or(&node_index)
}


// HELPER FUNCTIONS
// UCB1 = v(i) / n(i) + C * sqrt(ln(N) / n(i))
fn calculate_ucb(node_index: usize, tree: &Tree) -> f32 {
  let node = &tree.nodes[node_index];
  if node.visits == 0 {
    // insures unvisited nodes are prioritized
    return f32::INFINITY;
  }
  let exploitation = node.value as f32 / node.visits as f32;
  let exploration = C * ( (tree.nodes[tree.root].visits as f32).ln() / node.visits as f32 ).sqrt();
  exploitation + exploration
}

fn check_who_plays(state: &Board, first_player: &FieldData) -> FieldData {
  let (mut o_counter, mut x_counter) = (0, 0);

  for &field in state.iter().flatten() {
    match field {
      FieldData::O => o_counter += 1,
      FieldData::X => x_counter += 1,
      _ => {}
    }
  }

  if o_counter == x_counter {
    *first_player   // equal counts first player is next
  } else if x_counter > o_counter {
    FieldData::O
  } else {
    FieldData::X
  }
}

fn generate_optimal_move(state: &Board, player: &FieldData) -> usize {
  let opponent = match player {
    FieldData::X => FieldData::O,
    FieldData::O => FieldData::X,
    _ => FieldData::None
  };

  // 1. check for immediate win
  for i in 0..9 {
    if state[i / 3][i % 3] == FieldData::None {
      let mut temp = state.clone();
      temp[i / 3][i % 3] = *player;
      if check_for_winners(&temp) == *player {
        return i;
      }
    }
  }

  // 2. block opponents wins
  for i in 0..9 {
    if state[i / 3][i % 3] == FieldData::None {
      let mut temp = state.clone();
      temp[i / 3][i % 3] = opponent;
      if check_for_winners(&temp) == opponent {
        return i;
      }
    }
  }

  // 3. do a random move if no immediate wins
  random_bot_move(state)
}