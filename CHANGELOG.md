# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

---

## [1.0.0] – 2025-05-15

### Added
- First working version of shelltief_recorder
- Screen, mic, and cam recording via ffmpeg
- Project-based session archiving
- Auto fallback from iPhone to FaceTime camera

## [1.1.0] – 2025-05-17

### Added
- Configurable `DEVICE_1` variable for external camera name (e.g. iPhone via Continuity Camera)
- Graceful fallback if `current/` directory is not empty: prompt to continue or archive into `fail/`
- `recording_message` function for clear start/stop instructions
- Option parsing (`-h`) with a minimal usage screen

### Changed
- Converted `list_devices` from unused function to inline comment reference
- `user_continues` now includes line breaks for cleaner output
- Refactored `setup` logic for clarity and better failure handling
- Replaced verbose heredocs with concise inline comments

## [1.2.0] - 2025-05-17

### Added
- Configurable `THRESHOLD` variable to ensure there is enough space to record
- Creation and/or update of a `recording_sizes.log` file to record the sessions sizes
