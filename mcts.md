## Monte Carlo Tree Search (MCTS)

The Monte Carlo Tree Search (MCTS) algorithm is implemented to allow the AI to make strategic moves by simulating numerous possible game outcomes and evaluating the best possible action through statistical analysis.

### Description
MCTS consists of four main phases that are repeated until a computational limit is reached (can be time or number of iterations):

1. **Selection**: The algorithm starts at the root node and navigates the tree using the Upper Confidence Bound (UCB1) formula. This balances exploitation (choosing moves that have performed well) and exploration (trying out new moves).
2. **Expansion**: If the selected node has not been visited before, it is expanded by generating all possible moves from that state.
3. **Simulation (Rollout)**: A random or heuristic-based playout is conducted from the newly expanded node to simulate a game outcome (to terminal state).
4. **Back-propagation**: The result of the simulation is propagated back up the tree, updating the value and visit count of each node along the path (up to the root).

### How It Works
- The AI starts by initializing a tree with the root node representing the current board state.
- The tree expands dynamically as more simulations are performed, refining its strategy over time.
- Each node keeps track of the number of times it has been visited and the cumulative score of its outcomes.
- The UCB1 formula is used to determine the best node to explore, ensuring a balance between trying promising moves and discovering new possibilities.
- After a predefined number of simulations, the AI selects the move corresponding to the child node with the highest value.

MCTS is particularly effective in games with large search spaces, as it does not require an exhaustive evaluation of all possible moves. Instead, it focuses on high-potential moves and improves decision-making based on statistical outcomes over repeated simulations.