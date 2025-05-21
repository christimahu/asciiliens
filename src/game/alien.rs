// asciiliens/src/game/alien.rs

//! This module defines the `Alien` struct and its associated logic,
//! representing an enemy alien within the game. It handles alien movement,
//! collision detection with blasts, and explosion animations.

use super::blast::Blast;
use crate::util::constants::{
    ALIEN_DESIGNS, ALIEN_HEIGHT, ALIEN_WIDTH, BLAST_CHAR, EXPLOSION_STAGE_1, EXPLOSION_STAGE_2,
    EXPLOSION_STAGE_3, EXPLOSION_STAGE_4, GAME_WIDTH,
};
use rand::Rng; // Import `Blast` from the parent `game` module.

/// Represents an alien enemy in the game.
///
/// An `Alien` has a position (`x`, `y`), a status (`alive`), a visual
/// `design`, and an `explosion_frame` to manage its destruction animation.
#[derive(Debug, Clone, Copy)]
pub struct Alien {
    /// The x-coordinate of the alien's top-left corner.
    x: u16,
    /// The y-coordinate of the alien's top-left corner.
    y: u16,
    /// A boolean indicating whether the alien is currently alive.
    /// An alien is `alive` until its explosion animation completes.
    alive: bool,
    /// The 2x2 ASCII character design for this specific alien.
    /// This is chosen randomly upon creation from `ALIEN_DESIGNS`.
    design: [char; 4],
    /// The current frame of the explosion animation.
    /// - `0`: The alien is not exploding.
    /// - `1` to `4`: The alien is in an explosion animation stage (progressing through `EXPLOSION_STAGE_X`).
    /// - `5`: The explosion animation is complete, and the alien is ready for removal.
    explosion_frame: u8,
}

impl Alien {
    /// Creates a new `Alien` instance at a specified position with a randomly chosen design.
    ///
    /// The alien's design is selected from the `ALIEN_DESIGNS` constant using the
    /// provided random number generator.
    ///
    /// # Arguments
    /// * `x` - The initial x-coordinate of the alien's top-left corner.
    /// * `y` - The initial y-coordinate of the alien's top-left corner.
    /// * `rng` - A mutable reference to any type that implements the `rand::Rng` trait,
    ///   used for selecting a random alien design.
    ///
    /// # Returns
    /// A new `Alien` instance.
    pub fn new(x: u16, y: u16, rng: &mut impl Rng) -> Self {
        Self {
            x,
            y,
            alive: true,
            // Select a random alien design from the predefined array.
            design: ALIEN_DESIGNS[rng.gen_range(0..ALIEN_DESIGNS.len())],
            explosion_frame: 0, // All aliens start not exploding.
        }
    }

    /// Creates a new `Alien` instance with explicit properties, primarily for testing.
    ///
    /// This constructor is useful in test scenarios where the alien's initial state
    /// needs to be precisely controlled.
    ///
    /// # Arguments
    /// * `x` - The initial x-coordinate.
    /// * `y` - The initial y-coordinate.
    /// * `alive` - Whether the alien is initially alive.
    /// * `design` - The 2x2 character design.
    /// * `explosion_frame` - The initial explosion frame.
    ///
    /// # Returns
    /// A new `Alien` instance.
    #[cfg(test)] // This function is only compiled when running tests
    pub fn new_for_test(
        x: u16,
        y: u16,
        alive: bool,
        design: [char; 4],
        explosion_frame: u8,
    ) -> Self {
        Self {
            x,
            y,
            alive,
            design,
            explosion_frame,
        }
    }

    /// Returns the alien's current x-coordinate.
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Returns the alien's current y-coordinate.
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Sets the alien's y-coordinate.
    /// Used for internal modifications, e.g., when alien moves down.
    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    /// Returns whether the alien is alive.
    pub fn alive(&self) -> bool {
        self.alive
    }

    /// Sets the alien's alive status.
    /// Used for internal modifications, e.g., after explosion completes.
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }

    /// Returns the current explosion frame of the alien.
    pub fn explosion_frame(&self) -> u8 {
        self.explosion_frame
    }

    /// Sets the alien's explosion frame.
    /// Used for internal modifications, e.g., when hit by a blast.
    pub fn set_explosion_frame(&mut self, frame: u8) {
        self.explosion_frame = frame;
    }

    /// Increments the alien's explosion frame by one.
    /// Used for animating the explosion.
    pub fn increment_explosion_frame(&mut self) {
        self.explosion_frame += 1;
    }

    /// Returns the alien's design.
    /// Used for testing and display.
    pub fn design(&self) -> [char; 4] {
        self.design
    }

    /// Moves the alien one unit to the left.
    ///
    /// The movement is constrained by the left edge of the game screen,
    /// ensuring the alien never moves out of bounds (x-coordinate cannot go below 0).
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    /// Moves the alien one unit to the right.
    ///
    /// The movement is constrained by the right edge of the game screen,
    /// ensuring the alien's rightmost part (`x + ALIEN_WIDTH`) does not
    /// exceed `GAME_WIDTH`.
    pub fn move_right(&mut self) {
        if self.x < GAME_WIDTH - ALIEN_WIDTH {
            self.x += 1;
        }
    }

    /// Moves the alien one unit downwards.
    ///
    /// This method simply increments the alien's `y` coordinate.
    pub fn move_down(&mut self) {
        self.y += 1;
    }

    /// Checks if a `Blast`'s position overlaps with the alien's bounding box.
    ///
    /// This method also ensures that the alien is `alive` and not currently
    /// in an `explosion_frame` (as an exploding alien cannot be hit again).
    ///
    /// # Arguments
    /// * `blast` - A reference to the `Blast` to check collision against.
    ///
    /// # Returns
    /// `true` if the blast is within the alien's bounds and the alien is
    /// alive and not exploding, `false` otherwise.
    pub fn collides_with_blast(&self, blast: &Blast) -> bool {
        self.alive // Only collide if the alien is alive.
            && self.explosion_frame == 0 // Only collide if the alien is not currently exploding.
            // Check if the blast's x-coordinate is within the alien's horizontal span.
            && blast.x() >= self.x
            && blast.x() < self.x + ALIEN_WIDTH
            // Check if the blast's y-coordinate is within the alien's vertical span.
            && blast.y() >= self.y
            && blast.y() < self.y + ALIEN_HEIGHT
    }

    /// Generates the two lines of display strings for the alien,
    /// accounting for its original design or current explosion animation stage.
    ///
    /// If `explosion_frame` is 0, the alien's original `design` is returned.
    /// Otherwise, `BLAST_CHAR` (`*`) characters are substituted into the design
    /// based on the current `explosion_frame` and `EXPLOSION_STAGE_X` constants.
    ///
    /// # Returns
    /// A tuple containing two `String`s: (top row of characters, bottom row of characters).
    pub fn display_strings(&self) -> (String, String) {
        if self.explosion_frame == 0 {
            // If the alien is not exploding, return its original design.
            (
                format!("{}{}", self.design[0], self.design[1]),
                format!("{}{}", self.design[2], self.design[3]),
            )
        } else {
            // If the alien is exploding, determine which parts of its design
            // should be replaced by `BLAST_CHAR` (`*`).
            let explosion_stages = [
                EXPLOSION_STAGE_1,
                EXPLOSION_STAGE_2,
                EXPLOSION_STAGE_3,
                EXPLOSION_STAGE_4,
            ];
            // Calculate the current stage index (explosion_frame 1 maps to index 0, etc.).
            let stage_idx = (self.explosion_frame - 1) as usize;
            let current_stage = explosion_stages[stage_idx];

            // Determine characters for the top row based on the current explosion stage.
            let top_left = if current_stage[0] == 1 {
                BLAST_CHAR
            } else {
                self.design[0]
            };
            let top_right = if current_stage[1] == 1 {
                BLAST_CHAR
            } else {
                self.design[1]
            };
            // Determine characters for the bottom row based on the current explosion stage.
            let bottom_left = if current_stage[2] == 1 {
                BLAST_CHAR
            } else {
                self.design[2]
            };
            let bottom_right = if current_stage[3] == 1 {
                BLAST_CHAR
            } else {
                self.design[3]
            };

            (
                format!("{}{}", top_left, top_right),
                format!("{}{}", bottom_left, bottom_right),
            )
        }
    }
}
