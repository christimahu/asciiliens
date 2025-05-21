// asciiliens/src/main.rs

//! # ASCIIliens Game Executable
//!
//! This is the main entry point for the ASCIIliens game. It is responsible for:
//! - Initializing the terminal for raw mode and alternate screen display.
//! - Managing the main game loop, including handling user input.
//! - Drawing the game state on each frame.
//! - Handling game session flow, including the intro screen and play-again prompts.
//! - Cleaning up the terminal state upon exiting.

use asciiliens::{show_game_end_screen, show_intro_screen, Game, GameEvent, GameState};
use crossterm::{
    cursor::{self, MoveTo}, // `cursor` module for cursor visibility and positioning.
    event::{self, Event, KeyCode}, // `event` module for reading keyboard input.
    execute,
    queue,             // `execute` and `queue` for sending commands to the terminal.
    style::ResetColor, // `style` for terminal styling, like resetting colors.
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen}, // `terminal` for screen control.
};
use std::io::{self, Write}; // Standard I/O traits for interacting with the terminal. // Import core game logic and display functions from the library.

/// The main function, serving as the entry point of the ASCIIliens game application.
///
/// This function sets up the game's execution environment, runs the main game loop,
/// and ensures proper cleanup of the terminal state.
///
/// # Returns
/// An `io::Result<()>` indicating whether the program executed successfully or encountered an I/O error.
fn main() -> io::Result<()> {
    let mut stdout = io::stdout(); // Obtain a mutable handle to the standard output.

    // The outer loop allows the player to start a new game session after one ends.
    'game_loop: loop {
        // Display the introductory screen, which includes game instructions and a "Ready?" prompt.
        // This function handles its own terminal setup and input for the intro sequence.
        show_intro_screen(&mut stdout)?;

        // After the intro, prepare the terminal for the main game.
        // - `EnterAlternateScreen`: Switches to a fresh, clear terminal buffer.
        // - `Hide`: Hides the cursor to provide a cleaner game display.
        execute!(stdout, EnterAlternateScreen, cursor::Hide,)?;
        // Enable raw mode, which allows direct, unbuffered input from the keyboard
        // and bypasses line buffering, critical for real-time game input.
        terminal::enable_raw_mode()?;
        // Clear the entire alternate screen to ensure a clean game area.
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        // Create a new instance of the game, resetting its state for a fresh session.
        let mut game = Game::new();

        // The inner loop represents a single game session.
        'session_loop: loop {
            // Clear the screen at the beginning of each frame to draw the updated game state.
            // Move the cursor to (0,0) to start drawing from the top-left.
            queue!(
                stdout,
                terminal::Clear(terminal::ClearType::All),
                MoveTo(0, 0)
            )?;

            // Draw all game entities (player, blasts, aliens) to the buffered output.
            game.draw(&mut stdout)?;
            // Flush the buffer to send all drawing commands to the terminal for immediate display.
            stdout.flush()?;

            // Read player input (or detect other events) to determine the next `GameEvent`.
            let event = match event::read() {
                Ok(Event::Key(key_event)) => {
                    // Map keyboard input to specific `GameEvent`s.
                    match key_event.code {
                        KeyCode::Left => GameEvent::MoveLeft,
                        KeyCode::Right => GameEvent::MoveRight,
                        KeyCode::Char(' ') => GameEvent::Fire,
                        KeyCode::Char('q') | KeyCode::Esc => GameEvent::Quit, // 'q' or Esc key to quit.
                        _ => GameEvent::AdvanceFrame, // Any other key simply advances the game frame.
                    }
                }
                Ok(_) => {
                    // If an event other than a key press occurs (e.g., mouse event, resize),
                    // treat it as an `AdvanceFrame` to keep the game progressing.
                    GameEvent::AdvanceFrame
                }
                Err(e) => {
                    // If there's an error reading an event, print it and signal a quit event.
                    eprintln!("Error reading event: {:?}", e);
                    GameEvent::Quit
                }
            };

            // Update the game state based on the processed event.
            game.update(event);

            // Check the game's current state to decide if the session loop should end.
            match game.state() {
                GameState::Win | GameState::GameOver | GameState::Quit => {
                    // If the game is won, lost, or quit, break out of the current session loop.
                    break 'session_loop;
                }
                GameState::Playing => { /* Game is ongoing, continue the session loop. */ }
            }
        }

        // After a game session concludes, clean up the terminal state.
        // - `Show`: Makes the cursor visible again.
        // - `disable_raw_mode`: Restores standard terminal input buffering.
        // - `LeaveAlternateScreen`: Returns to the original terminal buffer.
        // - `ResetColor`: Resets any applied terminal colors.
        execute!(stdout, cursor::Show,)?;
        terminal::disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen, ResetColor)?;

        // Display the game end screen and ask the player if they want to play again.
        // `show_game_end_screen` handles its own input for the play-again prompt.
        let play_again_prompt_result =
            show_game_end_screen(&mut stdout, game.state(), game.score())?;

        // If the player chooses not to play again, exit the outer game loop, ending the application.
        if !play_again_prompt_result {
            break 'game_loop;
        }

        // If `play_again_prompt_result` is true, the 'game_loop' continues,
        // starting a new game session from the top.
    }

    Ok(())
}
