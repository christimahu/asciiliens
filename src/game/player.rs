// asciiliens/src/game/player.rs

//! This module defines the `Player` struct and implements its associated behavior,
//! representing the player's ship within the game.

use super::alien::Alien;
use crate::util::constants::{
    ALIEN_HEIGHT, ALIEN_WIDTH, GAME_HEIGHT, GAME_WIDTH, PLAYER_SHIP_ART, PLAYER_WIDTH,
    PLAYER_Y_OFFSET,
}; // Import `Alien` from the parent `game` module.

/// Represents the player's spaceship in the game.
///
/// The `Player` has a horizontal position (`x`) and is always positioned
/// at a fixed vertical level near the bottom of the screen.
#[derive(Debug, Clone, Copy)]
pub struct Player {
    /// The x-coordinate of the player's center.
    /// This value is clamped within the game boundaries.
    x: u16,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    /// Creates a new `Player` instance, positioned at the horizontal center
    /// of the game screen.
    ///
    /// # Returns
    /// A new `Player` instance ready for gameplay.
    pub fn new() -> Self {
        Player { x: GAME_WIDTH / 2 }
    }

    /// Creates a new `Player` instance with a specific x-coordinate, primarily for testing.
    ///
    /// This constructor is useful in test scenarios where the player's initial position
    /// needs to be explicitly set.
    ///
    /// # Arguments
    /// * `x` - The desired x-coordinate for the player's center.
    ///
    /// # Returns
    /// A new `Player` instance.
    #[cfg(test)] // This function is only compiled when running tests
    pub fn new_for_test(x: u16) -> Self {
        Player { x }
    }

    /// Returns the player's current x-coordinate.
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Moves the player's ship one unit to the left.
    ///
    /// The movement is constrained by the left edge of the game screen,
    /// ensuring the player's ship never moves out of bounds. The player's
    /// leftmost visual part (`x - PLAYER_WIDTH / 2`) is considered for the boundary.
    pub fn move_left(&mut self) {
        if self.x > PLAYER_WIDTH / 2 {
            self.x -= 1;
        }
    }

    /// Moves the player's ship one unit to the right.
    ///
    /// The movement is constrained by the right edge of the game screen,
    /// ensuring the player's ship never moves out of bounds. The player's
    /// rightmost visual part (`x + PLAYER_WIDTH / 2 - 1`) is considered for the boundary.
    pub fn move_right(&mut self) {
        if self.x < GAME_WIDTH - PLAYER_WIDTH / 2 - 1 {
            self.x += 1;
        }
    }

    /// Generates the string representation of the player's ship for rendering.
    ///
    /// This string includes the `PLAYER_SHIP_ART` constant directly.
    ///
    /// # Returns
    /// A `String` containing the ASCII art representation of the player.
    pub fn display_string(&self) -> &str {
        // Changed return type from String to &str
        PLAYER_SHIP_ART
    }

    /// Returns the fixed y-coordinate of the player's ship.
    ///
    /// The player is always positioned two rows from the bottom of the game screen.
    ///
    /// # Returns
    /// The `u16` y-coordinate where the player's ship is drawn.
    pub fn y_pos(&self) -> u16 {
        GAME_HEIGHT - PLAYER_Y_OFFSET // Player is always PLAYER_Y_OFFSET rows from the bottom
    }

    /// Checks for a geometric overlap between the player's bounding box
    /// and an `Alien`'s bounding box.
    ///
    /// This method performs a purely positional check. The `alive` status or
    /// `explosion_frame` of the alien are not considered here;
    /// higher-level game logic (in the `Game` struct) is responsible for
    /// deciding if this geometric collision results in a game over.
    ///
    /// # Arguments
    /// * `alien` - A reference to the `Alien` to check collision against.
    ///
    /// # Returns
    /// `true` if the player and alien bounding boxes overlap, `false` otherwise.
    pub fn collides_with_alien(&self, alien: &Alien) -> bool {
        // Calculate the player's horizontal bounding box.
        let player_left = self.x.saturating_sub(PLAYER_WIDTH / 2);
        let player_right = self.x + PLAYER_WIDTH / 2 - 1;

        // Calculate the alien's horizontal bounding box.
        let alien_left = alien.x();
        let alien_right = alien.x() + ALIEN_WIDTH - 1;

        // The player occupies a single row vertically.
        let player_top = self.y_pos();
        let player_bottom = self.y_pos();

        // Calculate the alien's vertical bounding box.
        let alien_top = alien.y();
        let alien_bottom = alien.y() + ALIEN_HEIGHT - 1;

        // Check for horizontal overlap:
        // Player's left edge must be to the left of or inside alien's right edge, AND
        // Player's right edge must be to the right of or inside alien's left edge.
        let horizontal_overlap = (player_left <= alien_right) && (player_right >= alien_left);

        // Check for vertical overlap:
        // Player's top edge must be above or at alien's bottom edge, AND
        // Player's bottom edge must be below or at alien's top edge.
        let vertical_overlap = (player_top <= alien_bottom) && (player_bottom >= alien_top);

        horizontal_overlap && vertical_overlap
    }
}
