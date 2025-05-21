// asciiliens/src/lib.rs

//! # ASCIIliens Game Library
//!
//! This library provides the core logic and components for a text-based Space Invaders game.
//! It is designed with a clear separation of concerns, organizing game elements
//! into distinct modules for player, blasts, aliens, and display.
//!
//! The `Game` struct within the `game` module is the central orchestrator, managing
//! game state, updates, and interactions between entities.
//!
//! # Modules
//! - `game`: Contains the core game logic, including the `Game` struct and its entities (`Player`, `Blast`, `Alien`).
//! - `util`: Provides utility functions and global constants used throughout the game.
//! - `display`: Handles all aspects of rendering game elements and screens to the terminal.

// Declare modules to be part of this crate.
// These `mod` declarations make the sub-modules available to the rest of the crate.
pub mod display;
pub mod game;
pub mod util;

// Re-export key types and functions from sub-modules for easier access
// by the `main.rs` binary crate and other parts of the library.
/// Re-exports `show_intro_screen` and `show_game_end_screen` functions
/// from the `display` module for convenient access.
pub use display::{show_game_end_screen, show_intro_screen};
/// Re-exports the `Game` struct, `GameEvent` enum, and `GameState` enum
/// from the `game` module for convenient access.
pub use game::{Game, GameEvent, GameState};
/// Re-exports `GAME_HEIGHT` and `GAME_WIDTH` constants from `util::constants`
/// for convenient access.
pub use util::constants::{GAME_HEIGHT, GAME_WIDTH};

// --- Tests ---
/// Unit tests for the core game logic and entities within the `asciiliens` library.
#[cfg(test)]
mod tests {
    // Import necessary items from the parent crate and local modules for testing.
    use crate::game::alien::Alien;
    use crate::game::blast::Blast;
    use crate::game::player::Player;
    use crate::game::{Game, GameEvent, GameState};
    use crate::util::constants::{
        ALIEN_DESIGNS, ALIEN_HEIGHT, ALIEN_MOVE_DOWN_FREQ, GAME_WIDTH, INITIAL_SCORE, PLAYER_WIDTH,
    };
    // Specifically import `StepRng` for predictable random number generation in tests.
    use rand::rngs::mock::StepRng;
    // The `Rng` trait is implicitly used by `StepRng` methods, so an explicit `use rand::Rng;` is not required here.

    /// Helper function to create a mock random number generator for deterministic testing.
    ///
    /// This `StepRng` starts at 1 and increments by 1 for each random number requested.
    /// This ensures that alien designs and horizontal movements in tests are predictable.
    ///
    /// # Returns
    /// A new `StepRng` instance.
    fn mock_rng() -> StepRng {
        StepRng::new(1, 1)
    }

    /// Helper function to create a new `Game` instance specifically for testing purposes.
    ///
    /// This function sets up a game with an initial grid of aliens,
    /// similar to the main game, but ensures that alien initialization uses a mock RNG
    /// for predictable test outcomes.
    ///
    /// # Returns
    /// A new `Game` instance configured for testing.
    fn new_test_game() -> Game {
        let mut game = Game::new(); // Initialize with default ThreadRng
        game.aliens_mut().clear(); // Clear the aliens initialized by ThreadRng
        Game::initialize_aliens_for_game(game.aliens_mut(), &mut mock_rng()); // Re-initialize with predictable aliens using mock RNG
        game
    }

    /// Tests the initial position of a new player.
    #[test]
    fn test_player_new() {
        let player = Player::new();
        // The player should be initialized at the horizontal center of the game width.
        assert_eq!(player.x(), GAME_WIDTH / 2);
    }

    /// Tests the player's ability to move left and respects the left boundary.
    #[test]
    fn test_player_move_left() {
        let mut player = Player::new_for_test(10); // Use new test constructor
        player.move_left();
        // Player should move one unit to the left.
        assert_eq!(player.x(), 9);

        // Test boundary condition: player should not move past the left edge.
        let mut player_at_edge = Player::new_for_test(PLAYER_WIDTH / 2); // Use new test constructor
        player_at_edge.move_left();
        // Player's position should remain at the boundary.
        assert_eq!(
            player_at_edge.x(),
            PLAYER_WIDTH / 2,
            "Player should not move past left boundary."
        );
    }

    /// Tests the player's ability to move right and respects the right boundary.
    #[test]
    fn test_player_move_right() {
        let mut player = Player::new_for_test(10); // Use new test constructor
        player.move_right();
        // Player should move one unit to the right.
        assert_eq!(player.x(), 11);

        // Test boundary condition: player should not move past the right edge.
        let mut player_at_edge = Player::new_for_test(GAME_WIDTH - PLAYER_WIDTH / 2 - 1); // Use new test constructor
        player_at_edge.move_right();
        // Player's position should remain at the boundary.
        assert_eq!(
            player_at_edge.x(),
            GAME_WIDTH - PLAYER_WIDTH / 2 - 1,
            "Player should not move past right boundary."
        );
    }

    /// Tests that a blast moves upwards and correctly indicates when it goes off-screen.
    #[test]
    fn test_blast_move_up() {
        let mut blast = Blast::new(10, 5);
        // Blast should move up and still be on screen.
        assert!(blast.move_up());
        assert_eq!(blast.y(), 4);

        // Test boundary condition: blast at the top edge.
        let mut blast_at_top = Blast::new(10, 0);
        // Blast should indicate it's off-screen and not move further up.
        assert!(!blast_at_top.move_up());
        assert_eq!(
            blast_at_top.y(),
            0,
            "Blast should stay at 0 if already off screen."
        );
    }

    /// Tests the creation of a new alien, checking its initial position, status, and design.
    #[test]
    fn test_alien_new() {
        let mut rng = mock_rng();
        let alien = Alien::new(5, 5, &mut rng);
        // Verify initial position.
        assert_eq!(alien.x(), 5);
        assert_eq!(alien.y(), 5);
        // Verify initial status.
        assert!(alien.alive());
        // With a mock RNG, the design should be predictable (the first one in `ALIEN_DESIGNS`).
        assert_eq!(alien.design(), ALIEN_DESIGNS[0]);
    }

    /// Tests that an alien correctly moves one unit downwards.
    #[test]
    fn test_alien_move_down() {
        let mut alien = Alien::new_for_test(10, 5, true, ALIEN_DESIGNS[0], 0); // Use new test constructor
        alien.move_down();
        // Alien's y-coordinate should increase by 1.
        assert_eq!(alien.y(), 6);
    }

    /// Tests various scenarios for blast-alien collision detection.
    #[test]
    fn test_alien_collides_with_blast() {
        let alien = Alien::new_for_test(10, 5, true, ALIEN_DESIGNS[0], 0); // Use new test constructor

        // Test blast directly hitting the alien.
        let blast_hit = Blast::new(10, 5);
        assert!(alien.collides_with_blast(&blast_hit));

        // Test blast hitting the second character of the alien (still within its bounds).
        let blast_hit_offset = Blast::new(11, 5);
        assert!(alien.collides_with_blast(&blast_hit_offset));

        // Test blast hitting the second row of the alien (still within its bounds).
        let blast_hit_bottom = Blast::new(10, 6);
        assert!(alien.collides_with_blast(&blast_hit_bottom));

        // Test blast missing the alien (too far to the right).
        let blast_miss = Blast::new(12, 5);
        assert!(!alien.collides_with_blast(&blast_miss));

        // Test blast missing the alien (too far down).
        let blast_miss_y = Blast::new(10, 7);
        assert!(!alien.collides_with_blast(&blast_miss_y));

        // Test a dead alien: it should not collide with a blast.
        let dead_alien = Alien::new_for_test(10, 5, false, ALIEN_DESIGNS[0], 0); // Use new test constructor
        assert!(!dead_alien.collides_with_blast(&blast_hit));

        // Test an exploding alien: it should not collide with a new blast.
        let exploding_alien = Alien::new_for_test(10, 5, true, ALIEN_DESIGNS[0], 1); // Use new test constructor
        assert!(
            !exploding_alien.collides_with_blast(&blast_hit),
            "Exploding alien should not be hit by a new blast."
        );
    }

    /// Tests various scenarios for player-alien geometric collision detection.
    ///
    /// This test strictly checks for spatial overlap, regardless of alien's `alive` status
    /// or `explosion_frame`, as per the `Player::collides_with_alien` contract.
    #[test]
    fn test_player_collides_with_alien() {
        let player = Player::new_for_test(GAME_WIDTH / 2); // Use new test constructor
        let player_y = player.y_pos();

        // Test alien directly overlapping player's position.
        let alien_overlap = Alien::new_for_test(
            player.x().saturating_sub(PLAYER_WIDTH / 2),
            player_y,
            true,
            ALIEN_DESIGNS[0],
            0,
        ); // Use new test constructor
        assert!(
            player.collides_with_alien(&alien_overlap),
            "Should geometrically collide with any alien overlapping player."
        );

        // Test alien slightly offset horizontally, but still overlapping.
        let alien_overlap_x = Alien::new_for_test(
            player.x().saturating_sub(PLAYER_WIDTH / 2) + 1,
            player_y,
            true,
            ALIEN_DESIGNS[0],
            0,
        ); // Use new test constructor
        assert!(
            player.collides_with_alien(&alien_overlap_x),
            "Should geometrically collide with alien overlapping player horizontally."
        );

        // Test alien slightly offset vertically, but still overlapping (alien bottom overlaps player top).
        let alien_overlap_y = Alien::new_for_test(
            player.x().saturating_sub(PLAYER_WIDTH / 2),
            player_y - ALIEN_HEIGHT + 1,
            true,
            ALIEN_DESIGNS[0],
            0,
        ); // Use new test constructor
        assert!(
            player.collides_with_alien(&alien_overlap_y),
            "Should geometrically collide with alien overlapping player vertically."
        );

        // Test alien just above player, no vertical overlap.
        let alien_above = Alien::new_for_test(
            player.x(),
            player_y - ALIEN_HEIGHT,
            true,
            ALIEN_DESIGNS[0],
            0,
        ); // Use new test constructor
        assert!(
            !player.collides_with_alien(&alien_above),
            "Should not geometrically collide with alien just above player."
        );

        // Test alien to the side, no horizontal overlap.
        let alien_side = Alien::new_for_test(
            player.x() + PLAYER_WIDTH,
            player_y,
            true,
            ALIEN_DESIGNS[0],
            0,
        ); // Use new test constructor
        assert!(
            !player.collides_with_alien(&alien_side),
            "Should not geometrically collide with alien to the side of player."
        );

        // Test a dead alien: it should still geometrically collide if positions overlap,
        // as `collides_with_alien` only checks geometry.
        let dead_alien = Alien::new_for_test(
            player.x().saturating_sub(PLAYER_WIDTH / 2),
            player_y,
            false,
            ALIEN_DESIGNS[0],
            0,
        ); // Use new test constructor
        assert!(
            player.collides_with_alien(&dead_alien),
            "Should geometrically collide with a dead alien if positions overlap."
        );

        // Test an exploding alien: it should still geometrically collide if positions overlap.
        let exploding_alien = Alien::new_for_test(
            player.x().saturating_sub(PLAYER_WIDTH / 2),
            player_y,
            true,
            ALIEN_DESIGNS[0],
            1,
        ); // Use new test constructor
        assert!(
            player.collides_with_alien(&exploding_alien),
            "Should geometrically collide with an exploding alien if positions overlap."
        );
    }

    /// Tests that unleashing a blast correctly adds a blast and deducts score.
    #[test]
    fn test_game_fire_blast() {
        let mut game = new_test_game();
        game.blasts_mut().clear(); // Ensure no pre-existing blasts.
        let initial_score = game.score();

        game.update(GameEvent::Fire);

        // Verify that a blast was added and its position is correct.
        assert_eq!(game.blasts().len(), 1);
        assert_eq!(
            game.blasts()[0].x(),
            game.player().x(),
            "Blast X should match player X at fire."
        );
        // Blast moves up by one in the same update call, so it's at y_pos - 2.
        assert_eq!(game.blasts()[0].y(), game.player().y_pos() - 2);
        // Verify score deduction.
        assert_eq!(
            game.score(),
            initial_score - 1,
            "Unleashing should deduct 1 point."
        );
    }

    /// Tests that moving the player ship correctly deducts score.
    #[test]
    fn test_game_movement_score_deduction() {
        let mut game = new_test_game();
        let initial_score = game.score();

        game.update(GameEvent::MoveLeft);
        assert_eq!(
            game.score(),
            initial_score - 1,
            "Moving left should deduct 1 point."
        );

        let current_score = game.score();
        game.update(GameEvent::MoveRight);
        assert_eq!(
            game.score(),
            current_score - 1,
            "Moving right should deduct 1 point."
        );
    }

    /// Tests that blasts are correctly updated (move up) and removed when off-screen.
    #[test]
    fn test_game_update_blasts() {
        let mut game = new_test_game();
        game.blasts_mut().clear(); // Start with a clean slate for blasts.
        game.blasts_mut().push(Blast::new(10, 1)); // Blast near the top.
        game.blasts_mut().push(Blast::new(20, 5)); // Blast in the middle.

        game.update_blasts(); // Directly call the internal method for testing.

        // Only the blast that started at y=5 should remain (and move to y=4).
        assert_eq!(game.blasts().len(), 1, "Only blast at y=5 should remain.");
        assert_eq!(game.blasts()[0].y(), 4, "Blast should have moved up.");
    }

    /// Tests that collisions correctly initiate an alien's explosion animation
    /// and remove the hitting blast.
    #[test]
    fn test_game_handle_collisions_starts_explosion() {
        let mut game = new_test_game();
        game.aliens_mut().clear(); // Clear initial aliens to control the test.
        game.blasts_mut().clear(); // Clear initial blasts.

        // Add a specific alien and a blast to hit it.
        game.aliens_mut()
            .push(Alien::new_for_test(10, 5, true, ALIEN_DESIGNS[0], 0)); // Use new test constructor
        game.blasts_mut().push(Blast::new(10, 5));

        game.handle_collisions(); // Directly call the internal method.

        // Verify that the alien started exploding.
        assert_eq!(
            game.aliens()[0].explosion_frame(),
            1,
            "Alien should start exploding (frame 1)."
        );
        assert!(
            game.aliens()[0].alive(),
            "Alien should still be alive during explosion animation."
        );
        // Verify that the blast was removed.
        assert!(
            game.blasts().is_empty(),
            "Blast should be removed after hitting."
        );
        // Score should not be awarded yet, only after the explosion completes.
        assert_eq!(
            game.score(),
            INITIAL_SCORE,
            "Score should not change on hit, only after after explosion completes."
        );
    }

    /// Tests that the explosion animation progresses correctly through its stages
    /// and that score is awarded upon completion.
    #[test]
    fn test_game_update_explosions_animates_and_scores() {
        let mut game = new_test_game();
        game.aliens_mut().clear(); // Clear initial aliens.
                                   // Add an alien that is starting its explosion animation.
        game.aliens_mut()
            .push(Alien::new_for_test(10, 5, true, ALIEN_DESIGNS[0], 1)); // Use new test constructor
        let initial_score = game.score();

        // Advance explosion from frame 1 to 2.
        game.update_explosions();
        assert_eq!(game.aliens()[0].explosion_frame(), 2);
        assert!(game.aliens()[0].alive());
        assert_eq!(game.score(), initial_score);

        // Advance explosion from frame 2 to 3.
        game.update_explosions();
        assert_eq!(game.aliens()[0].explosion_frame(), 3);
        assert!(game.aliens()[0].alive());
        assert_eq!(game.score(), initial_score);

        // Advance explosion from frame 3 to 4.
        game.update_explosions();
        assert_eq!(game.aliens()[0].explosion_frame(), 4);
        assert!(game.aliens()[0].alive());
        assert_eq!(game.score(), initial_score);

        // Advance explosion from frame 4 to 5 (completion frame).
        game.update_explosions();
        assert_eq!(
            game.aliens()[0].explosion_frame(),
            5,
            "Alien should reach frame 5."
        );
        // Alien should now be marked as not alive and score should be awarded.
        assert!(
            !game.aliens()[0].alive(),
            "Alien should be marked as not alive after explosion completes."
        );
        assert_eq!(
            game.score(),
            initial_score + 250,
            "Score should increase by 250 after explosion completes."
        );
    }

    /// Tests that aliens correctly move downwards after a certain number of frames.
    #[test]
    fn test_game_alien_vertical_movement() {
        let mut game = new_test_game();
        // Ensure no aliens are exploding so they are eligible for movement.
        game.aliens_mut()
            .iter_mut()
            .for_each(|alien| alien.set_explosion_frame(0));
        let initial_y = game.aliens()[0].y();

        // Advance the game frames until aliens should move down according to `ALIEN_MOVE_DOWN_FREQ`.
        for _i in 0..ALIEN_MOVE_DOWN_FREQ {
            game.update(GameEvent::AdvanceFrame); // This increments `frame_counter`.
        }
        // Verify that the alien's y-position has increased by 1.
        assert_eq!(
            game.aliens()[0].y(),
            initial_y + 1,
            "Alien should have moved down."
        );
    }

    /// Tests the game's win condition: all aliens are defeated.
    #[test]
    fn test_game_win_condition() {
        let mut game = new_test_game();
        // Simulate defeating all aliens by marking them as not alive.
        game.aliens_mut()
            .iter_mut()
            .for_each(|alien| alien.set_alive(false));
        game.check_game_over_conditions(); // Directly call the internal method.
                                           // Game state should be `Win`.
        assert_eq!(game.state(), GameState::Win);
    }

    /// Tests the game over condition when an alien invades the player's space (reaches the bottom).
    #[test]
    fn test_game_over_alien_invaded() {
        let mut game = new_test_game();
        // Extract player's y-position before getting mutable borrow of aliens
        let player_y_pos = game.player().y_pos();

        // Move an alien to the row just above the player's ship, ready to invade.
        // Player is at `GAME_HEIGHT - 2`. Aliens are `ALIEN_HEIGHT` tall.
        // If alien.y + ALIEN_HEIGHT - 1 >= player.y_pos(), it's a collision.
        game.aliens_mut()[0].set_y(player_y_pos - ALIEN_HEIGHT + 1);
        game.check_game_over_conditions(); // Directly call the internal method.
                                           // Game state should be `GameOver`.
        assert_eq!(game.state(), GameState::GameOver);
    }

    /// Tests the game over condition when the player ship directly collides with an alien.
    #[test]
    fn test_game_over_player_collision() {
        let mut game = new_test_game();
        game.aliens_mut().clear(); // Clear initial aliens.

        // Extract player coordinates first to avoid mutable/immutable borrow conflict
        let player_x = game.player().x().saturating_sub(PLAYER_WIDTH / 2);
        let player_y_pos = game.player().y_pos();

        // Place a new alien directly on top of the player's position.
        game.aliens_mut().push(Alien::new_for_test(
            player_x,     // Align alien's left with player's left.
            player_y_pos, // Place alien at player's y-position.
            true,
            ALIEN_DESIGNS[0],
            0,
        )); // Use new test constructor
        game.check_game_over_conditions(); // Directly call the internal method.
                                           // Game state should be `GameOver`.
        assert_eq!(game.state(), GameState::GameOver);
    }
}
