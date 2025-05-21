// asciiliens/src/display/mod.rs

//! This module provides functions responsible for rendering various screens
//! and graphical elements of the ASCIIliens game to the terminal.
//! It abstracts away the low-level `crossterm` commands for display.

use crate::game::GameState;
use crate::util::constants::{
    FINAL_SCORE_LABEL, GAME_HEIGHT, GAME_WIDTH, INSTRUCTIONS_TEXT, INTRO_TITLE_ART, LOSE_ART,
    PLAY_AGAIN_PROMPT, READY_PROMPT, SCORING_TEXT, TAUNT_PHRASES, WIN_ART,
};
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

/// Displays a generic screen with a top ASCII art banner, a body of instructional text,
/// and a prompt at a specified line.
///
/// This function clears the entire screen, then prints the provided art, body text,
/// and prompt, ensuring all content is centered horizontally.
///
/// # Arguments
/// * `stdout` - A mutable reference to a `Write` implementor, typically `io::stdout()`.
/// * `top_art` - A slice of string slices representing the ASCII art banner to display at the top.
/// * `body_text` - A slice of string slices containing the main instructional or informational text.
/// * `prompt_line_y` - The y-coordinate (row) where the prompt string should be displayed.
/// * `prompt` - The prompt string to display (e.g., "Ready? [Y/n] ").
///
/// # Returns
/// An `io::Result<()>` indicating success or failure of the display operations.
pub fn show_screen<W: Write>(
    stdout: &mut W,
    top_art: &[&str],
    body_text: &[&str],
    prompt_line_y: u16,
    prompt: &str,
) -> io::Result<()> {
    // Clear the entire terminal screen and move the cursor to the top-left corner.
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Print each line of the top ASCII art, centering it horizontally.
    for line in top_art {
        // Calculate padding to center the line.
        let padded_line = if GAME_WIDTH as usize > line.len() {
            format!("{: ^width$}", line, width = GAME_WIDTH as usize)
        } else {
            // If the line is too long, just print it as is (will overflow if not handled by terminal).
            line.to_string()
        };
        queue!(stdout, Print(format!("{}\n", padded_line)))?;
    }
    // Add an extra line break for visual spacing after the art.
    queue!(stdout, Print("\n"))?;

    // Print each line of the body text, centering it horizontally.
    for line in body_text {
        // Calculate padding to center the line.
        let padded_line = if GAME_WIDTH as usize > line.len() {
            format!("{: ^width$}", line, width = GAME_WIDTH as usize)
        } else {
            // If the line is too long, print as is.
            line.to_string()
        };
        queue!(stdout, Print(format!("{}\n", padded_line)))?;
    }
    // Add an extra line break for visual spacing after the body text.
    queue!(stdout, Print("\n"))?;

    // Move the cursor to the calculated position for the prompt and print it.
    // The prompt is centered horizontally on the `prompt_line_y`.
    queue!(
        stdout,
        MoveTo((GAME_WIDTH - prompt.len() as u16) / 2, prompt_line_y),
        Print(prompt)
    )?;
    // Flush the buffer to ensure all queued commands are written to the terminal immediately.
    stdout.flush()?;
    Ok(())
}

/// Displays the game's introductory screen, including the title art, instructions,
/// scoring information, and a "Ready?" prompt. It also handles
/// cycling through taunt phrases if the player chooses not to start immediately.
///
/// This function loops until the player input 'Y' or 'y' to start the game.
/// If 'N' or 'n' is pressed, a taunt phrase is displayed and cycled through.
/// Other keys are ignored.
///
/// # Arguments
/// * `stdout` - A mutable reference to a `Write` implementor, typically `io::stdout()`.
///
/// # Returns
/// An `io::Result<()>` indicating success or failure of the display and input operations.
pub fn show_intro_screen<W: Write>(stdout: &mut W) -> io::Result<()> {
    let mut taunt_index = 0;
    // Flag to control if a taunt should be displayed. Initially false,
    // becomes true only after the first 'n' input.
    let mut show_taunt = false;

    // Determine the target width for the taunt borders. Use GAME_WIDTH for consistent width.
    let taunt_border_len = GAME_WIDTH as usize;
    let taunt_separator_top = ">>>>>".repeat((taunt_border_len / 5) + 1); // Repeat to fill width
    let taunt_separator_bottom = "<<<<<".repeat((taunt_border_len / 5) + 1); // Repeat to fill width

    'intro_loop: loop {
        // Display the main intro screen elements (art, basic instructions).
        // The prompt itself will be handled at the very end of this loop.
        show_screen(
            stdout,
            &INTRO_TITLE_ART,
            &INSTRUCTIONS_TEXT,
            GAME_HEIGHT - 1,
            "",
        )?; // Pass empty string for prompt initially.

        // Calculate the starting Y-position for the first dynamic text block (scoring).
        // This accounts for the art lines, and the blank lines added by `show_screen` *before* the prompt.
        let mut current_y = INTRO_TITLE_ART.len() as u16 + 1 /* blank after art */ +
                            INSTRUCTIONS_TEXT.len() as u16 + 1; /* blank after instructions */

        // Print the scoring information.
        for line in SCORING_TEXT.iter() {
            let padded_line = format!("{: ^width$}", line, width = GAME_WIDTH as usize);
            queue!(stdout, MoveTo(0, current_y), Print(padded_line))?;
            current_y += 1;
        }
        current_y += 1; // Add a blank line after scoring.

        // Handle the display of taunt phrases if `show_taunt` is active.
        if show_taunt {
            let current_taunt = TAUNT_PHRASES[taunt_index];

            // Print top separator.
            let padded_separator_top = format!(
                "{: ^width$}",
                &taunt_separator_top[..taunt_border_len],
                width = GAME_WIDTH as usize
            );
            queue!(stdout, MoveTo(0, current_y), Print(padded_separator_top))?;
            current_y += 1;

            // Print current taunt phrase.
            let padded_taunt = format!("{: ^width$}", current_taunt, width = GAME_WIDTH as usize);
            queue!(stdout, MoveTo(0, current_y), Print(padded_taunt))?;
            current_y += 1;

            // Print bottom separator.
            let padded_separator_bottom = format!(
                "{: ^width$}",
                &taunt_separator_bottom[..taunt_border_len],
                width = GAME_WIDTH as usize
            );
            queue!(stdout, MoveTo(0, current_y), Print(padded_separator_bottom))?;
            current_y += 1;
        } else {
            // If taunts are not yet active, ensure the taunt area is clear.
            // Clear the 3 lines where taunt & separators would go.
            for i in 0..3 {
                queue!(
                    stdout,
                    MoveTo(0, current_y + i),
                    Print(format!("{: <width$}", "", width = GAME_WIDTH as usize))
                )?;
            }
            current_y += 3; // Advance y for the cleared space.
        }
        current_y += 1; // Add a blank line before the READY_PROMPT.

        // Now, finally print the READY_PROMPT at the calculated current_y.
        queue!(
            stdout,
            MoveTo((GAME_WIDTH - READY_PROMPT.len() as u16) / 2, current_y),
            Print(READY_PROMPT)
        )?;

        // Position the cursor immediately after the "Ready? [Y/n] " prompt.
        queue!(
            stdout,
            MoveTo(
                (GAME_WIDTH - READY_PROMPT.len() as u16) / 2 + READY_PROMPT.len() as u16,
                current_y
            )
        )?;
        stdout.flush()?; // Flush to ensure everything is visible before reading input.

        // Wait for player input.
        // In a more complex application, input reading might be handled by a central event loop.
        // For this game's structure, direct input reading here is acceptable for simplicity.
        if let Ok(Event::Key(key_event)) = crossterm::event::read() {
            match key_event.code {
                KeyCode::Char(c) if c.eq_ignore_ascii_case(&'y') => break 'intro_loop, // Start game.
                KeyCode::Char(c) if c.eq_ignore_ascii_case(&'n') => {
                    show_taunt = true; // Activate taunts after the first 'n'.
                                       // Cycle to the next taunt phrase.
                    taunt_index = (taunt_index + 1) % TAUNT_PHRASES.len();
                }
                _ => { /* Ignore all other key presses and loop again. */ }
            }
        }
    }
    Ok(())
}

/// Displays the game end screen, showing whether the player won or lost,
/// their final score, and a "Play Again?" prompt.
///
/// This function waits for player input ('Y'/'y' to play again, 'N'/'n'/'Esc' to quit).
///
/// # Arguments
/// * `stdout` - A mutable reference to a `Write` implementor, typically `io::stdout()`.
/// * `game_state` - The final `GameState` (either `Win` or `GameOver`).
/// * `score` - The player's final score.
///
/// # Returns
/// An `io::Result<bool>`: `Ok(true)` if the player chooses to play again,
/// `Ok(false)` if they choose to quit.
pub fn show_game_end_screen<W: Write>(
    stdout: &mut W,
    game_state: GameState,
    score: i32,
) -> io::Result<bool> {
    // Determine which ASCII art and status message to display based on the `game_state`.
    let (art, status_message) = match game_state {
        GameState::Win => (&WIN_ART, "YOU WON! :) "),
        GameState::GameOver => (&LOSE_ART, "YOU LOST :( "),
        // This case should ideally not be reached if called only at game end.
        _ => (&WIN_ART, "GAME ENDED"),
    };

    // Format the final score string.
    let score_text = format!("{}: {}", FINAL_SCORE_LABEL, score);

    // Prepare the body text lines for the `show_screen` function.
    let body_lines: Vec<&str> = vec![
        status_message,
        &score_text,
        "", // Add an empty line for spacing.
    ];

    // Display the game end screen.
    // The prompt line is fixed at GAME_HEIGHT - 1.
    show_screen(stdout, art, &body_lines, GAME_HEIGHT - 1, PLAY_AGAIN_PROMPT)?;

    // Position the cursor immediately after the "Play again? [Y/n] " prompt.
    queue!(
        stdout,
        MoveTo(
            (GAME_WIDTH - PLAY_AGAIN_PROMPT.len() as u16) / 2 + PLAY_AGAIN_PROMPT.len() as u16,
            GAME_HEIGHT - 1
        )
    )?;
    stdout.flush()?; // Ensure cursor is positioned before reading input.

    // Loop to wait for valid player input ('Y'/'y' to play again, 'N'/'n'/'Esc' to quit).
    // Similar to `show_intro_screen`, direct input reading is used for simplicity here.
    let play_again = loop {
        if let Ok(Event::Key(key_event)) = crossterm::event::read() {
            match key_event.code {
                KeyCode::Char(c) if c.eq_ignore_ascii_case(&'y') => break true,
                KeyCode::Char(c) if c.eq_ignore_ascii_case(&'n') => break false,
                KeyCode::Esc => break false,
                _ => { /* Ignore other keys. */ }
            }
        }
    };
    Ok(play_again)
}
