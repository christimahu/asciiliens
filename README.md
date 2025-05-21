# ASCIIliens

ASCII + Aliens = ASCIIliens -- Turn-Based ASCII Arcade Action.

This project is a nostalgic recreation of one of my very first programs, originally written in Borland C++ when I was just 13 years old. Back then, it ran on an incredible machine: an x386sx, boasting a whopping 4MB of RAM (with an expansion board!), and displayed its glory on a CRT monitor that knew only green letters – no fancy graphics, just pure ASCII.

This rewrite in Rust is a fun way to keep that original program alive, bringing its core logic and turn-based gameplay into a modern, safe, and performant language.

**Features:**
* Classic Space Invaders-inspired gameplay
* Turn-based action, where each move or shot advances the game
* Retro ASCII art visuals
* Player movement (left/right) and firing bullets
* Alien movement and explosion animations
* Scoring system

**How to Play:**
1.  **Navigate:** Use the `LEFT` and `RIGHT` arrow keys to move your ship (║_||_║).
2.  **Fire:** Press `SPACE` to shoot bullets (*).
3.  **Strategic Action:** Each action (move or fire) advances one game frame. You cannot move and fire in the same 'turn'.
4.  **Score:** Start with 100 points, lose 1 point for each movement or shot, and gain 250 points for destroying an ASCIIlien.
5.  **Win/Lose:** Defeat all aliens to win, or lose if aliens reach the bottom of the screen or collide with your ship.

**Building and Running:**

To compile and run ASCIIliens, you will need the Rust toolchain installed.

```bash
# Clone the repository (if you haven't already)
git clone [https://github.com/your-username/asciiliens.git](https://github.com/your-username/asciiliens.git)
cd asciiliens

# Build the project
cargo build --release

# Run the game
cargo run --release
```

**Dependencies:**
* `crossterm` for terminal handling
* `rand` for random number generation

**License:**
This project is Licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.  This means you are free to use, modify, and distribute this game, or incorporate its code into your own projects, provided you adhere to the terms of the license, including maintaining its open-source nature.

**Contributing:**
This project is a labor of love, aiming to capture the authentic feel of a very first program. While direct feature contributions to this specific repo might be limited to maintain its original spirit, please visit another Rust repository of mine, such as [DevRS](https://github.com/christimahu/devrs), to collaborate on more active development!
