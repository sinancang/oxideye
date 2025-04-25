# (WIP) rust-daemon

This is a toy project for a Rust daemon to record and log computer use metrics.\
The main goal is to develop some knowledge and working experience with Rust 🦀

## Planned Features
- Auto-start at boot
- Listen to OS and log PC usage stats such as:
    - keystrokes
    - mouse clicks
    - mouse miles
    - pixel data
    - audio data
    - screen time
- Logs them periodically either to a file or a light DB
- CLI command to fetch all logged metrics and print them to impress your friends 😄

## Stretch Goals
- Share metrics across different computers by synchronizing with a metrics server 
    - Requires authentication, scheduled uploads and downloads, highly available server
- Enable extensions and third-party integrations via "extensions" (e.g. Spotify listen hours, Apple watch health metrics, Teams messages, etc.)