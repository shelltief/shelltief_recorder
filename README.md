# Shelltief Recorder

> This is a *read-only* mirror. The original version lives on [Gitlab](https://gitlab.com/shelltief/shelltief_recorder/)

A `Rust` binary to record your screen, webcam, and mic
simultaneously, all form the terminal.

> :warning: This script is `macOS` only as it uses `AvFoundation` backend
> (a `macOS` specific implementation of screen capture)

> :warning: For the binary to run properly, you need to have
> `ffmpeg` installed and in your path. You can find more information
> about ffmpeg on their [website](https://ffmpeg.org/download.html)

Built for:
- Developers who build and want to **document their process**
- Builders who prefer **terminal rituals over clicking buttons**
- People who want to record **directly to disk** with zero overhead

## Install

The crate is *not yet* on `crate.io`, however you can install it
with `cargo install --path .` from within the directory

## Stream

This binary uses the concept of `stream` to work.
A stream is defined by four variables:
- Video input
- Audio input (optionnal)
- Ouput file (for now, the screen only allows `mp4` output format)
- Recording path (optionnal) -> The path to a directory to which
the recordings should be saved

## Continuity Camera

This binary can also record your iPhone through
[`Continuity Camera`](https://support.apple.com/en-us/102546)

This means that from your terminal window, you can launch a recording
on your iPhone screen

## Features

- Records any stream you give it as an input
- Saves to a `current/` directory, which is then auto-archived to `0/`, `1/`, `2/`, ...
- If `current/` already exists, wipes out its contents (so beware of
checking what happens between two recordings)
- **Preview mode**: Uses `ffplay` to allow you to preview the
streams instead of recording
- Manages process PIDs for clean starts and stops
- Fully terminal-driven

## Usage

```bash
shellrecord [options] -- project_name
```

The project name argument is mandatory, and the `--` also to separate
the options from the project_name

Each option has a long and a short version, feel free to use the
one that's most natural to you

By default:
- Recordings are saved under the current directory `.` or `${PWD}`
    - Each project gets its own folder
    - Each session gets stored in `0/`, `1/`, `2/`, etc.
**Automatically**

> :warning: This script doesn’t currently check available storage — if your drive fills up
> mid-session, the recording will fail silently.
> Be mindful of free space, especially when recording long sessions.


1. Run the script with your project name
2. Press any key to start recording
3. Press any key to stop recording
4. `current/` is automatically archived to a numbered folder

## Directory Requirements

The `project_name` directory **must** be created within the
`project_path` directory.

## Suggested Workflow

1. List Available Devices

```bash
shellrecord -l
```

You'll then get an ouput like this one:

```
---Video Devices---
Name: FaceTime HD Camera -- Index: 0
Name: Nokia de Thibault Camera -- Index: 1
Name: Capture screen 0 -- Index: 2
Name: Capture screen 1 -- Index: 3
---Audio Devices---
Name: Microsoft Teams Audio -- Index: 0
Name: MacBook Air Microphone -- Index: 1
Name: Nokia de Thibault Microphone -- Index: 2
```

You can then record the streams

For example:
```bash
shellrecord --stream 'FaceTime HD Camera' 'MacBook Air Microphone' face.mp4 --stream 'Capture screen 1' screen.mp4 -- project_name
```

Will record:
- FaceTime Camera with sound from macbook microphone
- External display named 'Capture screen 1' with no sound
Both will be recorded into the current path.

<blockquote>

This command could also be written as :
```bash
shellrecord --stream 0 1 face.mp4 --stream 3 screen.mp4 -- project_name
```

</blockquote>

You can also use the `-P` (or `--preview`), to preview the streams
before running the script

```bash
shellrecord --stream 0 1 face.mp4 --stream 3 screen.mp4 -P -- project_name
```

## AI Training / Commercial Use

This project is released under a [GPL-3.0 license](LICENSE.md) with the following restrictions:

**AI Training Prohibited**
You may NOT use this code to train, fine-tune, or power any machine learning system, including Copilot, ChatGPT, or similar tools.

**Commercial Use Requires a License**
If you intend to use this in a closed-source or for-profit project, you must contact me for a commercial license.

> Built by [Shelltief](https://shelltief.sh) to document work, in a
> smooth way
