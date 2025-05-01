# oxideye 👁️‍🗨️

"oxideye" is a work in progress that records and logs **computer peripheral usage statistics** (think mouse distance, key/button press, etc.).

The main goal in developing this for me is:
1. Learn Rust development 🦀
2. Discover how many times my mouse goes to the moon and back every week🌕

---
Currently the program logs:
- Mouse distance (in pixels)
- Wheel spins
- Button presses
- Key presses

---
Down the line I intend to add:
- CLI tool to aggregate and view the stats
- Notifications on milestones (when stats exceed certain thresholds)
- Stat sharing across multiple devices

## Usage

1. Clone the repository:
    ```bash
    git clone https://github.com/sinancang/oxideye.git
    cd oxideye
    ```

2. Run the project:
    ```bash
    cargo run
    ```

Note: Ensure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
