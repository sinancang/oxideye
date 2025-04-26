# (WIP) rust-daemon

This is a toy project for a Rust daemon to record and log computer use metrics.\
The main goal is to develop some knowledge and working experience with Rust 🦀

## Planned Features
- Listen to OS and log PC usage stats such as: total mouse_distance, wheel_distance, button/key presses, etc. (Partially done)
- Log them periodically either to a file or a light DB (TODO)
- CLI command to fetch all logged metrics and print them to impress your friends 😄 (TODO)

## Stretch Goals
- Share metrics across different computers by synchronizing with a metrics server 
    - Requires highly available metrics client, authentication, scheduled uploads and downloads
- Enable third-party integrations via "extensions" (e.g. Spotify listen hours, Apple watch health metrics, Teams messages, etc.)

## Usage

To run the Rust daemon, follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/rust-daemon.git
    cd rust-daemon
    ```

2. Run the project:
    ```bash
    cargo run
    ```

Note: Ensure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).
