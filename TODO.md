# Tasks

This is a list of early tasks which should probably be implemented in the following order:
- [ ] Represent the board and pieces as a `struct` of bitboards (i.e. `u64` integers).
- [ ] Figure out how we want to generate moves. On the fly, or precomputed and stored?
- [ ] Given a board and player turn, generate all possible moves at that level.
- [ ] Implement minimum subset of UCI commands for later testing and UCI GUI interfacing:
      https://www.chessprogramming.org/Sequential_Probability_Ratio_Test#Minimum_UCI_Requirements
