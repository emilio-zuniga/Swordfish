# Branch Tasks

### Board Representation:
- [] Test mailbox array
- [] Add piece-square list

### Move Generation:
- [] Shift from Copy/Make to Make/Unmake paradigm
- [] Test move making and unmaking with mementos
- [x] ~~Add Zobrist hash generation for instances of `GameManager`~~
- [] Incorporate hash updates within make/unmake move
- [] Research Transposition Tables:
  - What does the DS look like?
  - What information needs to be encoded within the structure?
  - What behavior should the TT have as deeper searches are performed? (Updates to previous values?)

### Search & Evaluation:
- [] Incorporate iterative deepening (for timed searched)
- [] Incorporate move ordering (for prioritizing search of better moves)

### UCI:
- [] Add handling of more keywords
- [] Create instances of a Game given 'ucinewgame' command

### General:
- [] Flesh out README.md
