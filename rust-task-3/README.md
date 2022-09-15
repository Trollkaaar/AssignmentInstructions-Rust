# DD1337 Week 3-4

## Chess Project

The time has come to flex your newly found Rust skills by writing a killer ass chess engine. This and the next weeks' homework is to implement the logic for a chess game:
- whose turn it is
- how pieces can move
- [check](https://en.wikipedia.org/wiki/Check_(chess))
- [promotion](https://en.wikipedia.org/wiki/Promotion_(chess))

_(optional)_ Implement special moves:
- [castling](https://en.wikipedia.org/wiki/Castling)
- [en passant](https://en.wikipedia.org/wiki/En_passant)
- [checkmate](https://en.wikipedia.org/wiki/Checkmate)
- [dead postion](https://en.wikipedia.org/wiki/Glossary_of_chess#dead_position)

The idea is that this library could be used by a chess GUI application. Therefore, privatise all resources in you library but the following public functions in the public struct `Game`:

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game. |
| `pub fn set_promotion(&mut self, _piece: &str) -> ()` | Set the piece type that a pawn becames following a promotion. |
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves(&self, _position: &str) -> Option<Vec<String>>` | If a piece is standing on the given tile, return all possible new positions of that piece. Don't forget to the rules for check. _(optional)_ Don't forget to include en passent and castling. |

Positions are given as strings with the format `"<file><rank>"`.

Your program also exports an enumerators `GameState` with the values:
- `InProgress`, 
- `Check`,
- `GameOver`, 
- _(optional)_ `Checkmate` and
- _(optional)_ `DeadPosition`.

#### Expansion

A GUI application could also make use of enumeratorss such as `Colour` and `Piece`. You may also like to make changes to the above API depeding on your board representation. If your library API do not reflect the documentation above, write your own complementary documentation in your repository's `README.md` file.

### Prepare assignment

1) Create a repository named `<KTH_ID>-chess` under the `INDAPlus22` organisation and clone it.
2) Navigate into your newly created repository and initialise a Rust library crate with the following command.
```bash
cargo init --lib
```

See the template crate for help with code setup.

### Grading

Since your crate is of type library, we cannot simply test it by running it. Instead, test your application with Rust unit tests. 

The grading on this assignment is based on how well the tests demonstrates the full functionality and game mechanics of your chess engine. The tests are expected to not fail, and to print a representation of the board in the case of move demonstrations. **Test at least all of your implemented functionality from the lists above to prove implementation.**

Run your unit tests with comments by entering the following command in your terminal:
```
cargo test -- --nocapture --test-threads=1
```

In addition to unit tests, all your public structures, functions, constants, and enumeratorss **must** have well written documentation comments.

### Implementation Ideas

The complexity of your implementation will depend how you store piece positions.

#### Bit Maps
A chess board has 64 squares, which allows for bitmaps using `u64`.

```rs
struct Game {
    state: GameState,
    black: u64, 
    white: u64,
    kings: u64,
    queens: u64,
    bishops: u64,
    knights: u64,
    rooks: u64,
    pawns: u64,
    //...
}
```
You can further use an enumerators to index a sequence of bit maps.
```rs
enum Piece {
    King = 0, Queen = 1, Bishop = 2, Knight = 3, Rook = 4, Pawn = 5
}

struct Game {
    state: GameState,
    black: u64, 
    white: u64,
    board: [u64; 6],
    //...
}
```

#### Enumerators
Rust enumerators allows for polymorphism, unlike structs. However, an enumerators cannot hold fields, which are needed for colour and position.
```rs
enum Piece {
    King, Queen, Bishop, Knight, Rook, Pawn
}

impl Piece {
    fn get_possible_moves(&self, /*...*/) -> /*...*/ {
        match self {
            Piece::King => /*...*/,
            Piece::Queen => /*...*/,
            //...
        }
    }
}

struct Game {
    state: GameState,
    black: u64, 
    white: u64,
    board: [Option<Piece>; 64],
    //...
}
```

#### Structures
If you look to offload the colour or/and positions from the `Game` struct, make use of a `Piece` structure.
```rs
struct Piece {
    colour: /*...*/,
    role: /*...*/,
    //...
}

impl Piece {
    //...
}

struct Game {
    state: GameState,
    board: [Option<Piece>; 64],
    //...
}
```
Observe that a structure can be an abstraction of a single value, or no value at all.
```rs
enum Colour {
    White, Black
}

struct Piece(Colour);

impl Piece {
    //...
}
```
```rs
struct Piece;

impl Piece {
    //...
}
```
Move logic can in turn be further offloaded to a `Role` enumerator. 

To keep smaller structures, the structures could be role-specific and make use of traits. Note that role-specific structures can be messy and requires additional knowledge of Rust memory management.
```rs
trait Piece {
    fn get_possible_moves(&self, /*...*/) -> /*...*/;
    //...
}

struct King {/*...*/}

impl Piece for King {

}

//...

struct Game {
    state: GameState,
    board: [Option<Box<dyn Piece>>; 64]
    //...
}
```

#### Logic-Only

Positioning and colours can be offloaded totally to the GUI by returning the functions instead of performing the logic. 

For example:
```rs
enum Piece {
    King, Queen, Bishop, Knight, Rook, Pawn
}

impl Piece {
    fn get_possible_moves(&self) -> 
        impl Fn(/*...*/) -> /*...*/ {

        match self {
            //...
        }
    }
}

//...

struct Game {
    state: GameState,
    //...
}
```