// asciiliens/src/game/mod.rs

//! This module encapsulates the core game logic and defines the main entities
//! and state transitions for the ASCIIliens game.

// Declare sub-modules that are part of the `game` module.
pub mod alien;
pub mod blast;
pub mod player;

// Import necessary crates and modules for game operations.
use crate::util::constants::{
    ALIEN_HEIGHT, ALIEN_MOVE_DOWN_FREQ, ALIEN_WIDTH, BLAST_CHAR, GAME_HEIGHT, GAME_WIDTH,
    INITIAL_SCORE, PLAYER_WIDTH,
};
use rand::Rng; // Required for random number generation, e.g., alien movement.
use std::io::{self, Write}; // Standard I/O traits for drawing.

// Import public structs from sub-modules for direct use within `game` module.
use self::alien::Alien;
use self::blast::Blast;
use self::player::Player;

// --- Game State Enums ---

/// Represents the overall state of the game.
///
/// This enum dictates what the game is currently doing, influencing
/// how updates are processed and what screen is displayed.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    /// The game is actively running, accepting input and updating entities.
    Playing,
    /// The player has successfully defeated all aliens.
    Win,
    /// The player has lost the game (e.g., aliens reached the bottom, or collision).
    GameOver,
    /// The player has explicitly chosen to quit the game.
    Quit,
}

/// Represents an event that can occur in the game, typically originating
/// from user input or internal game mechanics.
///
/// These events drive the game's state updates in the `Game::update` method.
#[derive(Debug, Clone, Copy)]
pub enum GameEvent {
    /// Instructs the player ship to move one column to the left.
    MoveLeft,
    /// Instructs the player ship to move one column to the right.
    MoveRight,
    /// Instructs the player ship to unleash a blast.
    Fire,
    /// Instructs the game to terminate.
    Quit,
    /// Advances the game simulation by one frame without specific player action.
    /// This is a fallback for non-action inputs, ensuring aliens still move.
    AdvanceFrame,
}

// --- Main Game Struct ---

/// The central structure holding the entire state of the ASCIIliens game.
///
/// This struct manages the player, blasts, aliens, game progression,
/// score, and random number generation for dynamic behaviors.
#[derive(Debug)]
pub struct Game {
    player: Player,
    blasts: Vec<Blast>,
    aliens: Vec<Alien>,
    frame_counter: u64,
    game_state: GameState,
    rng: rand::rngs::ThreadRng, // Keep this as ThreadRng for main game logic
    score: i32,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Creates a new `Game` instance, initializing all game entities to their
    /// starting positions and states.
    ///
    /// This constructor sets up the player, populates the initial grid of aliens,
    /// resets the frame counter, and sets the initial game state to `Playing`.
    ///
    /// # Returns
    /// A new `Game` instance ready for the first frame of gameplay.
    pub fn new() -> Self {
        let mut game = Self {
            player: Player::new(),
            blasts: Vec::new(),
            aliens: Vec::new(), // Initialize aliens as empty for now
            frame_counter: 0,
            game_state: GameState::Playing,
            rng: rand::thread_rng(), // Default RNG for actual gameplay
            score: INITIAL_SCORE,    // The game starts with INITIAL_SCORE points.
        };
        // Now, call the associated function to populate aliens, borrowing disjoint fields
        Game::initialize_aliens_for_game(&mut game.aliens, &mut game.rng);
        game
    }

    /// Populates the given `aliens` vector with an initial grid of aliens
    /// using the provided RNG.
    ///
    /// This is an associated function (static method) that operates on mutable
    /// references to the aliens vector and an RNG, allowing for flexible initialization.
    /// Made `pub(crate)` for testability.
    pub(crate) fn initialize_aliens_for_game(aliens: &mut Vec<Alien>, rng: &mut impl Rng) {
        for row in 0..3 {
            for col in 0..10 {
                // Aliens are spaced out: `col * 6` for horizontal spacing,
                // `row * 3` for vertical spacing, plus offsets for initial position.
                aliens.push(Alien::new(col * 6 + 10, row * 3 + 3, rng)); // Use the passed rng
            }
        }
    }

    /// Returns the current state of the game.
    ///
    /// # Returns
    /// A `GameState` enum indicating if the game is `Playing`, `Win`, `GameOver`, or `Quit`.
    pub fn state(&self) -> GameState {
        self.game_state
    }

    /// Returns the player's current score.
    ///
    /// # Returns
    /// An `i32` representing the score.
    pub fn score(&self) -> i32 {
        self.score
    }

    /// Returns an immutable reference to the player.
    pub fn player(&self) -> &Player {
        &self.player
    }

    /// Returns an immutable slice of active blasts.
    pub fn blasts(&self) -> &[Blast] {
        &self.blasts
    }

    /// Returns a mutable slice of active blasts.
    /// Used for operations like `clear()` or `push()`.
    pub fn blasts_mut(&mut self) -> &mut Vec<Blast> {
        &mut self.blasts
    }

    /// Returns an immutable slice of aliens.
    pub fn aliens(&self) -> &[Alien] {
        &self.aliens
    }

    /// Returns a mutable slice of aliens.
    /// Used for operations like `clear()` or `push()`.
    pub fn aliens_mut(&mut self) -> &mut Vec<Alien> {
        &mut self.aliens
    }

    /// Returns the current frame counter.
    pub fn frame_counter(&self) -> u64 {
        self.frame_counter
    }

    /// Returns a mutable reference to the game's RNG.
    #[allow(dead_code)] // Only used in tests
    pub(crate) fn rng_mut(&mut self) -> &mut rand::rngs::ThreadRng {
        &mut self.rng
    }

    /// Updates the entire game state based on a `GameEvent`.
    ///
    /// This is the central function for the game loop, responsible for:
    /// 1. Incrementing the frame counter.
    /// 2. Processing player input (`GameEvent`).
    /// 3. Updating blast positions.
    /// 4. Handling collisions between blasts and aliens.
    /// 5. Advancing alien explosion animations.
    /// 6. Handling alien horizontal and vertical movements.
    /// 7. Checking for game over or win conditions.
    ///
    /// Game updates only occur if the `game_state` is `Playing`.
    ///
    /// # Arguments
    /// * `event` - The `GameEvent` that triggered this update (e.g., player move, fire, quit).
    pub fn update(&mut self, event: GameEvent) {
        // If the game is no longer in the `Playing` state, no further updates should occur.
        if self.game_state != GameState::Playing {
            return;
        }

        // Increment the global frame counter for timing game events.
        self.frame_counter += 1;

        // Process the specific `GameEvent` received.
        match event {
            GameEvent::MoveLeft => {
                self.player.move_left();
                self.score = self.score.saturating_sub(1); // Deduct 1 point for each movement.
            }
            GameEvent::MoveRight => {
                self.player.move_right();
                self.score = self.score.saturating_sub(1); // Deduct 1 point for each movement.
            }
            GameEvent::Fire => {
                self.fire_blast();
                self.score = self.score.saturating_sub(1); // Deduct 1 point for each blast unleashed.
            }
            GameEvent::Quit => {
                // If a quit event occurs, set the game state to `Quit`.
                self.game_state = GameState::Quit;
            }
            GameEvent::AdvanceFrame => {
                // No specific action for `AdvanceFrame` other than incrementing the counter.
                // This allows alien movement and other time-based events to still occur.
            }
        }

        // Perform sequence of update steps for all game entities.
        self.update_blasts();
        self.handle_collisions();
        self.update_explosions(); // Update any ongoing alien explosion animations.
        self.update_alien_movement();
        self.check_game_over_conditions(); // Check if the game has ended (win, lose).
    }

    /// Unleashes a new `Blast` from the player's current horizontal position.
    ///
    /// The blast is initialized just above the player's ship.
    fn fire_blast(&mut self) {
        self.blasts.push(Blast::new(
            self.player.x(),
            self.player.y_pos().saturating_sub(1), // Blast starts one row above player.
        ));
    }

    /// Updates the vertical positions of all active blasts and removes any that
    /// have gone off-screen (reached the top edge of the game area).
    ///
    /// This method iterates through all blasts, moves them up, and then filters
    /// out those that are no longer visible.
    pub(crate) fn update_blasts(&mut self) {
        // Iterate through blasts using a mutable iterator.
        // For each blast, attempt to move it up.
        for blast in &mut self.blasts {
            blast.move_up();
        }
        // After all blasts have attempted to move, retain only those that are still on screen.
        // `retain` here correctly takes an immutable reference to check the `y` position.
        self.blasts.retain(|blast| blast.y() > 0);
    }

    /// Detects and handles collisions between blasts and aliens.
    ///
    /// When a blast collides with an alive and non-exploding alien:
    /// - The alien begins its explosion animation (`explosion_frame` is set to 1).
    /// - The blast is marked for removal (its `y` is set to 0), and then filtered out.
    ///
    /// Each blast can only hit one alien.
    pub(crate) fn handle_collisions(&mut self) {
        // Iterate through all blasts.
        self.blasts.iter_mut().for_each(|blast| {
            // Only process blasts that are still on screen (not already marked for removal).
            if blast.y() > 0 {
                // Find the first alien that this blast collides with.
                for alien in self.aliens.iter_mut() {
                    if alien.collides_with_blast(blast) {
                        // If an alien is hit, start its explosion animation.
                        alien.set_explosion_frame(1);
                        // Mark the blast for removal by moving it off-screen.
                        blast.set_y(0);
                        break; // A blast can only hit one alien.
                    }
                }
            }
        });
        // Remove all blasts that have hit an alien or gone off-screen.
        self.blasts.retain(|blast| blast.y() > 0);
    }

    /// Advances explosion animation frames for aliens and awards score
    /// when an explosion sequence completes.
    ///
    /// Aliens progress through 4 explosion stages. Upon completing the 4th stage
    /// (moving to frame 5), the alien is marked as `!alive` and points are awarded.
    pub(crate) fn update_explosions(&mut self) {
        // Refactored from .for_each to a standard for loop for clarity on side effects.
        for alien in self.aliens.iter_mut() {
            if alien.explosion_frame() > 0 && alien.explosion_frame() < 5 {
                // Increment the explosion frame.
                alien.increment_explosion_frame();
                // If the explosion animation has just completed (reached frame 5),
                // mark the alien as not alive and award points.
                if alien.explosion_frame() == 5 {
                    alien.set_alive(false); // Alien is now fully exploded and no longer active.
                    self.score = self.score.saturating_add(250); // Award points for destroying an alien.
                }
            }
        }
    }

    /// Manages the movement of aliens, both horizontally and vertically.
    ///
    /// - **Horizontal Movement**: In each frame, one random *alive and non-exploding*
    ///   alien will attempt to move horizontally towards the player.
    /// - **Vertical Movement**: All *alive and non-exploding* aliens move one row down
    ///   periodically, based on `ALIEN_MOVE_DOWN_FREQ`.
    fn update_alien_movement(&mut self) {
        // Horizontal movement: One random alien moves towards the player.
        // Collect mutable references to aliens that are alive and not currently exploding.
        let mut movable_aliens: Vec<&mut Alien> = self
            .aliens
            .iter_mut()
            .filter(|alien| alien.alive() && alien.explosion_frame() == 0)
            .collect();

        if !movable_aliens.is_empty() {
            // Select a random alien from the filtered list.
            let alien_to_move_idx = self.rng.gen_range(0..movable_aliens.len()); // Use self.rng
            let alien_to_move = &mut movable_aliens[alien_to_move_idx];

            // Determine if the alien should move left or right to get closer to the player.
            // Player's horizontal center is roughly `player.x()`.
            let player_effective_x = self.player.x();

            if alien_to_move.x() < player_effective_x.saturating_sub(PLAYER_WIDTH / 2)
                && alien_to_move.x() < GAME_WIDTH - ALIEN_WIDTH
            {
                // Move right if the alien is significantly to the left of the player.
                alien_to_move.move_right();
            } else if alien_to_move.x() > player_effective_x + PLAYER_WIDTH / 2 - 1
                && alien_to_move.x() > 0
            {
                // Move left if the alien is significantly to the right of the player.
                alien_to_move.move_left();
            }
            // If the alien is horizontally aligned with the player, it will not move horizontally.
        }

        // Vertical movement: All relevant aliens move down periodically.
        if self.frame_counter % ALIEN_MOVE_DOWN_FREQ == 0 {
            // Iterate over all aliens and move down only those that are alive and not exploding.
            self.aliens
                .iter_mut()
                .filter(|alien| alien.alive() && alien.explosion_frame() == 0)
                .for_each(|alien| alien.move_down());
        }
    }

    /// Checks for conditions that would end the game (win or game over).
    ///
    /// This method performs the following checks in order:
    /// 1. **Win Condition**: If all aliens are no longer alive (either destroyed or fully exploded).
    /// 2. **Lose Condition (Invasion)**: If any alive, non-exploding alien reaches or crosses the player's row.
    /// 3. **Lose Condition (Collision)**: If the player's ship geometrically collides with any
    ///    alive, non-exploding alien.
    ///
    /// Aliens that have finished their explosion animation (`explosion_frame == 5`)
    /// are filtered out before checking win conditions.
    pub(crate) fn check_game_over_conditions(&mut self) {
        // First, clean up aliens that have completed their explosion animation.
        // An alien is fully gone when `alive` is false and its `explosion_frame` is 5 (or greater).
        self.aliens
            .retain(|alien| alien.alive() || alien.explosion_frame() < 5);

        // Win condition: Check if there are no more alive aliens.
        // An alien is considered "active" (not fully gone) if it's alive OR still exploding.
        if self.aliens.iter().all(|alien| !alien.alive()) {
            self.game_state = GameState::Win;
            return; // Game has been won, no need for further checks.
        }

        // Lose condition 1: Any active alien invades the player's space (reaches or crosses player's Y-position).
        // The player is at `GAME_HEIGHT - 2`. Aliens are `ALIEN_HEIGHT` tall.
        // A collision occurs if the alien's bottom edge (`a.y() + ALIEN_HEIGHT - 1`)
        // is at or below the player's top edge (`self.player.y_pos()`).
        if self.aliens.iter().any(|alien| {
            alien.alive()
                && alien.explosion_frame() == 0
                && (alien.y() + ALIEN_HEIGHT - 1) >= self.player.y_pos()
        }) {
            self.game_state = GameState::GameOver;
            return; // Game is over due to invasion.
        }

        // Lose condition 2: Player-Alien direct collision.
        // This checks for physical contact between the player's ship and any active alien.
        if self.aliens.iter().any(
            |alien| {
                alien.alive()
                    && alien.explosion_frame() == 0
                    && self.player.collides_with_alien(alien)
            }, // Uses the player's collision method.
        ) {
            self.game_state = GameState::GameOver; // Game is over due to player collision.
        }
    }

    /// Draws the current game state to the provided `Write` target.
    ///
    /// This function renders the player, blasts, aliens, and game status/score.
    /// It does not clear the screen; screen clearing is handled by the main loop
    /// before each draw call for a smooth update.
    ///
    /// # Arguments
    /// * `stdout` - A mutable reference to a `Write` implementor, typically `io::stdout()`.
    ///
    /// # Returns
    /// An `io::Result<()>` indicating success or failure of the drawing operations.
    pub fn draw<W: Write>(&self, stdout: &mut W) -> io::Result<()> {
        use crossterm::cursor::MoveTo;
        use crossterm::queue;
        use crossterm::style::Print; // `queue!` is used for batching commands for efficiency.

        // Draw the player's ship.
        // The player's `x` is its center, so `saturating_sub(PLAYER_WIDTH / 2)`
        // gets the starting x-coordinate for drawing its full width.
        queue!(
            stdout,
            MoveTo(
                self.player.x().saturating_sub(PLAYER_WIDTH / 2),
                self.player.y_pos()
            ),
            Print(self.player.display_string())
        )?;

        // Draw all active blasts.
        for blast in self.blasts() {
            queue!(
                stdout,
                MoveTo(blast.x(), blast.y()),
                Print(BLAST_CHAR) // Use the global `BLAST_CHAR` constant.
            )?;
        }

        // Draw all aliens that are either alive or in an explosion animation.
        for alien in self.aliens() {
            if alien.alive() || alien.explosion_frame() > 0 {
                // Only draw if active in some way.
                let (top_str, bottom_str) = alien.display_strings(); // Get 2-line alien art.
                queue!(
                    stdout,
                    MoveTo(alien.x(), alien.y()), // Top row of alien.
                    Print(top_str),
                    MoveTo(alien.x(), alien.y() + 1), // Bottom row of alien.
                    Print(bottom_str)
                )?;
            }
        }

        // Draw the current score at the bottom-left of the screen.
        let score_line_y = GAME_HEIGHT - 1; // The row for the score and status messages.
        queue!(
            stdout,
            MoveTo(0, score_line_y),
            Print(format!("Score: {} ", self.score()))
        )?;

        // Determine and draw the game status message.
        // This message provides feedback to the player (e.g., instructions, win/lose).
        let game_status_message = match self.game_state {
            GameState::Playing => "Press 'q' to quit, 'left/right' arrows to move, 'space' to fire. Hit any key to advance.",
            GameState::Win => "YOU WON! :) ",
            GameState::GameOver => "YOU LOST :( ",
            GameState::Quit => "Quitting...",
        };
        queue!(
            stdout,
            // Position the status message to the right of the score.
            // Start at column 12 to provide some padding after the score.
            MoveTo(12, score_line_y),
            Print(game_status_message)
        )?;

        Ok(())
    }
}
