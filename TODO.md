# Tasks

This is a list of early tasks which should probably be implemented in the following order:
- [x] Represent the board and pieces as a `struct` of bitboards (i.e. `u64` integers).
- [x] Figure out how we want to generate moves. On the fly, or precomputed and stored? PRECOMPUTED!
- [x] Port Sebastian Lague's FEN string handling, and complete the `from_fen()` and `to_fen()` functions in [bitboard.rs](src/bitboard.rs).
- [x] Implement functions within `GameMaster` for tracking data not represented by `BitBoard` (castling rights, etc.)
- [ ] Given a board and player turn, generate all possible *pseudo-legal* moves at that level.
- [ ] Implement minimum subset of UCI commands for later testing and UCI GUI interfacing:
      https://www.chessprogramming.org/Sequential_Probability_Ratio_Test#Minimum_UCI_Requirements


Current Subtasks:
**MoveTable**:
 - [ ] Add each side's castling moves to lookup table (can be hardcoded)
 - [ ] Add each side's pawn pushes, attacks, and promotions to lookup table
 - [ ] Edit black_pawn_moves so that it only generates pushes (without promotions)
 - [ ] Edit white_pawn_moves so that it only generates pushes (without promotions)
 - [ ] Add black_pawn_captures and have it generate attacking moves
 - [ ] Add white_pawn_captures and have it generate attacking moves
 - [ ] Add black_pawn_promotions and have it generate promoting pushes
       (just check w (position & 0x00ff0000_00000000) == position)
 - [ ] Add white_pawn_promotions and have it generate promoting pushes
       (just check w (position & 0x00000000_0000ff00) == position)
 - [ ] Document get_moves
 - [ ] Document get_moves_as_bitboard
**GameManager**:
- [ ] implement fn get_board(piece: PieceType, color: Color) -> u64
      Note: will just call and pass arguments to BitBoard's get_board
**BitBoard**:
- [ ] implement fn get_board(piece: PieceType, color: Color) -> u64