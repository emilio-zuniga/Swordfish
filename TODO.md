# Tasks

This is a list of early tasks which should probably be implemented in the following order:
- [x] Represent the board and pieces as a `struct` of bitboards (i.e. `u64` integers).
- [x] Figure out how we want to generate moves. On the fly, or precomputed and stored? PRECOMPUTED!
- [ ] Given a board and player turn, generate all possible *pseudo-legal* moves at that level.
- [ ] Port Sebastian Lague's FEN string handling, and complete the `from_fen()` and `to_fen()` functions in [bitboard.rs](src/bitboard.rs).
- [ ] Implement minimum subset of UCI commands for later testing and UCI GUI interfacing:
      https://www.chessprogramming.org/Sequential_Probability_Ratio_Test#Minimum_UCI_Requirements
