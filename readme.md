# Tic Tac Toe - eframe Rust Application

## Overview

This project is a simple Tic Tac Toe game implemented using the eframe and egui libraries in Rust. The game allows two players to take turns and play on a 3x3 grid, with a graphical user interface for interaction. The application highlights the current player and announces the winner once the game is concluded.

## Features

Two-player mode: Players X and O take turns to play.
Graphical User Interface: The game uses eframe and egui for rendering the UI.
Current Player Display: Shows the current player taking the turn.
Winner Announcement: Announces the winner once the game is won.
Restart Button: Allows players to restart the game at any point.
Colored Moves: Player X's moves are displayed in red, and Player O's moves are displayed in blue.

## User Interface

### Main Screen
Title: The game starts with a heading "Tic Tac Toe".
Current Player Indicator: Displays the current player (X or O).
Game Board: A 3x3 grid where players can click to place their moves.
Restart Button: A button to reset the game to its initial state.

### Board Interaction
Players click on the grid cells to place their moves.
Cells will show "X" or "O" based on the player's turn.
The current player is indicated above the grid.
Once a player wins, a message displays the winner.

## Code Structure

### TicTacToe Struct
board: A 3x3 array representing the game board.
current_player: The current player, either 'X' or 'O'.
winner: Option type indicating the winner, if any.

### Implementations
Default Implementation

Initializes the game board, sets the current player to 'X', and sets the winner to None.
epi::App Implementation

update: The main function to update the UI elements. It handles rendering the game board, player moves, and the restart button.
name: Returns the name of the application.
TicTacToe Methods

check_winner: Checks the board to determine if the current player has won the game.

## How to Run

Ensure you have Rust and Cargo installed.
Clone the repository:
sh
Copy code
git clone <repository-url>
Navigate to the project directory:
sh
Copy code
cd tic-tac-toe-eframe
Run the application:
sh
Copy code
cargo run

## Dependencies

eframe
egui
These dependencies are specified in the Cargo.toml file.