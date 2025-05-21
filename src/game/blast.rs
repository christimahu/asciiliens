// asciiliens/src/game/blast.rs

//! This module defines the `Blast` struct and its associated behavior,
//! representing projectiles unleashed by the player.

/// Represents a blast unleashed by the player's ship.
///
/// A `Blast` has an `x` and `y` coordinate, determining its position
/// on the game screen.
#[derive(Debug, Clone, Copy)]
pub struct Blast {
    /// The x-coordinate (horizontal position) of the blast.
    x: u16,
    /// The y-coordinate (vertical position) of the blast.
    y: u16,
}

impl Blast {
    /// Creates a new `Blast` instance at a specified position.
    ///
    /// # Arguments
    /// * `x` - The initial x-coordinate of the blast.
    /// * `y` - The initial y-coordinate of the blast.
    ///
    /// # Returns
    /// A new `Blast` instance.
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Returns the blast's current x-coordinate.
    pub fn x(&self) -> u16 {
        self.x
    }

    /// Returns the blast's current y-coordinate.
    pub fn y(&self) -> u16 {
        self.y
    }

    /// Sets the blast's y-coordinate.
    /// This is primarily for internal modifications, e.g., marking for removal.
    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    /// Moves the blast upwards by one row.
    ///
    /// This method decrements the blast's `y` coordinate. It returns a boolean
    /// indicating whether the blast is still within the visible screen area
    /// (i.e., its `y` coordinate is greater than or equal to 0 after the move).
    ///
    /// # Returns
    /// `true` if the blast is still on screen after moving, `false` otherwise (if it reached y=0 and would move off).
    pub fn move_up(&mut self) -> bool {
        if self.y > 0 {
            self.y -= 1;
            true
        } else {
            false // Blast would go off screen if moved further up.
        }
    }
}
