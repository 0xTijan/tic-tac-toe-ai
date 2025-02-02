## Minimax Algorithm
The Minimax algorithm is a recursive decision-making technique used in two-player games. It simulates all possible moves, assigning scores to game states. The AI aims to maximize its own chances while minimizing the opponent's best options.

### How Minimax Works
The function starts by identifying all empty fields on the board. It then checks if the game is already in a terminal state (win, loss, or draw). If so, it returns a score based on the outcome:
- **AI wins**: Score of `1`
- **Human wins**: Score of `-1`
- **Draw**: Score of `0`

If the game is not in a terminal state, the function proceeds to simulate every possible move:
1. The current player makes a move on an empty field.
2. The function recursively calls itself to evaluate the opponent's response.
3. After the recursive evaluation, the board state is reset to its original state.
4. The results of each simulated play (score and move index) are stored in a list.

Once all possible moves have been simulated, the AI selects the best move based on the current player:
- **AI (Maximizing Player - O)**: Chooses the move with the highest score.
- **Human (Minimizing Player - X)**: Chooses the move with the lowest score.

The function ultimately returns the best move for the current player, ensuring optimal decision-making based on future outcomes.

Minimax guarantees optimal play but can be computationally expensive for larger game trees. Performance can be improved using **alpha-beta pruning** (not implemented), which eliminates unnecessary branches to speed up decision-making.