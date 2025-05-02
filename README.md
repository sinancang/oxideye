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

### Clone the repository:
```bash
git clone https://github.com/sinancang/oxideye.git
cd oxideye
```

### Run the project:
```bash
cargo run
```

### Let it run!
oxideye will, by default write logs to `oxideye/telem.log` every `100000` milliseconds. Both of these are configurable via modifying `config/default.toml`

### Debug Mode
Alternatively, run the program in debug mode to see every time the process is notified of a usage event, every time an update goes through, etc.
```bash
cargo run -- --log-level debug
```
