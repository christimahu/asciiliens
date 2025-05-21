// asciiliens/src/util/constants.rs

//! This module defines game-wide constants, including screen dimensions,
//! character designs, timing frequencies, and all ASCII art elements.

/// The width of the game screen in columns.
pub const GAME_WIDTH: u16 = 80;
/// The height of the game screen in rows.
pub const GAME_HEIGHT: u16 = 24;

/// The complete ASCII art string for the player's ship.
pub const PLAYER_SHIP_ART: &str = "║_||_║";
/// The character used to represent blasts.
pub const BLAST_CHAR: char = '*';

/// The frequency at which aliens move down, in game frames.
/// For example, a value of 10 means aliens move down every 10 frames.
pub const ALIEN_MOVE_DOWN_FREQ: u64 = 10;

/// The visual width of the player's ship in characters.
pub const PLAYER_WIDTH: u16 = 6;
/// The visual width of an alien in characters.
pub const ALIEN_WIDTH: u16 = 2;
/// The visual height of an alien in characters.
pub const ALIEN_HEIGHT: u16 = 2;

/// The fixed vertical offset from the bottom of the screen for the player's ship.
pub const PLAYER_Y_OFFSET: u16 = 2;

/// The starting score for a new game.
pub const INITIAL_SCORE: i32 = 100;

/// A collection of 2x2 ASCII character designs for various alien types.
/// Each inner array represents [top-left, top-right, bottom-left, bottom-right] characters.
pub const ALIEN_DESIGNS: [[char; 4]; 4] = [
    // Reverting to `as char` as per user feedback, as these values previously rendered desired graphical characters.
    // This relies on the terminal's interpretation of these specific extended ASCII values.
    [178 as char, 178 as char, 176 as char, 176 as char], // ▓▓, ░░ (Dark Shade, Light Shade)
    [219 as char, 219 as char, 223 as char, 223 as char], // █ █, ▄ ▄ (Full Block, Lower Half Block)
    [220 as char, 220 as char, 220 as char, 220 as char], // ▀▀, ▀▀ (Upper Half Block - a blocky one)
    [234 as char, 234 as char, 196 as char, 196 as char], // ΩΩ, ── (Omega, Box Drawing Light Horizontal)
];

/// Defines the stages of the alien explosion animation.
/// Each array indicates which of the 2x2 characters turn to '*' (1) or remain
/// their original design (0) at a given explosion frame.
/// The indices correspond to: [top-left, top-right, bottom-left, bottom-right].
pub const EXPLOSION_STAGE_1: [u8; 4] = [1, 0, 0, 0]; // Only top-left becomes '*'
/// The second stage of the alien explosion animation.
pub const EXPLOSION_STAGE_2: [u8; 4] = [1, 1, 0, 0]; // Top-left, top-right
/// The third stage of the alien explosion animation.
pub const EXPLOSION_STAGE_3: [u8; 4] = [1, 1, 1, 0]; // Top-left, top-right, bottom-left
/// The fourth and final stage of the alien explosion animation before disappearance.
pub const EXPLOSION_STAGE_4: [u8; 4] = [1, 1, 1, 1]; // All become '*'

// --- ASCII Art & Phrases ---

/// The ASCII art for the game's introductory title screen.
/// This is a simple block design for clarity and readability, ensuring proper border alignment.
pub const INTRO_TITLE_ART: [&str; 5] = [
    "╔══════════════════════════════════════════════════════════════════════════════╗",
    "║                      ASCII + Aliens = ASCIIliens                             ║",
    "╠══════════════════════════════════════════════════════════════════════════════╣",
    "║                  -- The ASCII Invasion Begins! --                            ║",
    "╚══════════════════════════════════════════════════════════════════════════════╝",
];

/// The basic instructional text displayed on the intro screen.
/// This section focuses on controls.
pub const INSTRUCTIONS_TEXT: [&str; 4] = [
    "Navigate your ship (║_||_║) using LEFT/RIGHT arrow keys.",
    "Press SPACE to fire blasts (*).",
    "Each action (move or fire) advances one game frame.",
    "Strategic action is key – you cannot move and fire in the same 'turn'!",
];

/// The scoring rules displayed on the intro screen.
pub const SCORING_TEXT: [&str; 4] = [
    "Scoring:",
    "- Start with 100 points.",
    "-1 point for each movement (left/right).",
    "+250 points for destroying an ASCIIlien.",
];

/// A collection of taunt phrases displayed when the player hesitates.
pub const TAUNT_PHRASES: [&str; 5] = [
    "Now is not the time for the timid, step up!",
    "Do you fear the ASCIIliens, cadet?",
    "Your pixelated courage is lacking! Try again.",
    "The fate of the terminal rests on your bold choice!",
    "A true hero would not hesitate. Are you a hero?",
];

/// The prompt displayed to ask the player if they are ready to start.
pub const READY_PROMPT: &str = "Ready? [Y/n] ";
/// The prompt displayed to ask the player if they want to play again.
pub const PLAY_AGAIN_PROMPT: &str = "Play again? [Y/n] ";

/// The ASCII art displayed when the player wins the game.
pub const WIN_ART: [&str; 5] = [
    "╔══════════════════════════════════════════════════════════════════════════════╗",
    "║                       CONGRATULATIONS, COMMANDER!                            ║",
    "║                 YOU HAVE REPELLED THE ASCII INVASION!                        ║",
    "╚══════════════════════════════════════════════════════════════════════════════╝",
    "                           VICTORY IS YOURS!                                    ",
];

/// The ASCII art displayed when the player loses the game.
pub const LOSE_ART: [&str; 5] = [
    "╔══════════════════════════════════════════════════════════════════════════════╗",
    "║                         MISSION FAILED!                                      ║",
    "║                  THE ASCII INVASION OVERWHELMED US!                          ║",
    "╚══════════════════════════════════════════════════════════════════════════════╝",
    "                           GAME OVER                                          ",
];

/// The label for displaying the final score.
pub const FINAL_SCORE_LABEL: &str = "Score:";
