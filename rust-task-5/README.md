# DD1337 Week 5

## Chess GUI Project

Congratulation ettan! You are now a proved owner of a killer ass chess engine. It's time to make it fabulous! Your assignment this week is to write a GUI applikation for a chess engine. The *CATCH* is that you cannot use your own chess engine(!). Therefore, if your chess engine implementation differs from the specifications stated in the `rust-task-3` instructions, write your own API specifications in a README markdown file and push it to your engine repository. 

Take inspiration from existing chess games or go crazy and make something rustacular!

### Prepare assignment

1) Create a repository named `<KTH_ID>-chess-gui` under the `INDAPlus21` organisation and clone it.
2) Navigate into your newly created repository and initialise a Rust application crate (like you did the first week).

See the template crates for help with code setup.

#### Game Engine

There are several Rust game engines to choose from. Shortlist example: https://blog.logrocket.com/5-rust-game-engines-consider-next-project/

See `./chess-gui-templates` for two templates in Good Games Easily (GGEZ) and Piston.

##### GGEZ and Linux

For the most pog of users, additional dependencies may have to be installed for GGEZ to interact properly with some hardware.

The following is known to solve most issues:
```
sudo apt install libasound2-dev libudev-dev pkg-config
```

##### GGEZ and Windows Subsystems for Linux

Unfortunately, GGEZ and WSL has been proven to be especially tricky. For example, if you are experiencing `'Failed to build context.: AudioError("Could not initialize sound system using default output device (for some reason)")'`, it is mentally sane to disenable the audio:
```rust
ContextBuilder::new()./*...*/.modules(conf::ModuleConf::default().audio(false));
```

#### Chess GUI template

Navigate in your command prompt/terminal to `./rust-task-5/chess-gui-templates/*-template`. Run the application to show a chess board with the game state shown in text. 

The `resources` directory contains image files for all chess pieces, as well as the application icon file. The chess piece image files are loaded into image structures; a gift from me to you. Switch out the image files if you prefer to render the pieces in a different style. 

To switch from the uncomplete `chess-template` engine repository as a dependency, to one of your comrades' state of the art creations, change the target URL in the `Cargo.toml` file.
```toml
chess_template = { git = "https://github.com/INDAPlus21/chess-template.git" }
```

### Grade

To pass this assignment, write a GUI application which allows you to both play and replay chess. 

Remember: Write neet code with elegant documentation.
