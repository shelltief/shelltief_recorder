# shelltief-recorder

A dead-simple Bash script to record your screen, webcam, and mic simultaneously — all from the terminal.

Built for:
- Developers who build and want to **document their process**
- Builders who prefer **terminal rituals over clicking buttons**
- People who want to record **directly to disk** with zero overhead

## ⚙️ Features

- Records your **screen** and **mic** (macOS only)
- Detects and records your **webcam**, configurable via `DEVICE_1` (e.g. iPhone via Continuity Camera)
- Saves to a `current/` directory, which is then auto-archived to `0/`, `1/`, `2/`, ...
- If `current/` already exists, prompts to continue or archive it to a `fail/` directory
- Manages process PIDs for clean starts and stops
- Fully terminal-driven — no GUI

## 🍏 macOS Only (for now)

This script uses Apple's `avfoundation` backend and expects `ffmpeg` installed (via Homebrew or similar).

To list available input devices:
```bash
ffmpeg -f avfoundation -list_devices true -i ""
```

The default iPhone camera name is set as:

```bash
DEVICE_1=Nokia
```

Modify it in the script to match your iPhone name. For example, if your iPhone is named
`iPhone de John` then you can change for:

```bash
DEVICE_1=iPhone
```

* `"FaceTime"` is used as the fallback webcam

🧠 **Tip:** Continuity Camera works only when your iPhone and Mac are on the same Wi-Fi, unlocked, and nearby. Once detected, the iPhone cam shows up wirelessly and can be used **without plugging it in.**

> :information_desk_person: No worries if you don't have an iPhone, Continuity Camera won't be
detected and the script will fall back to your camera as needed.

> :bulb: As you might have guessed, if another camera is detected by the `avfoundation`
backend, you will be able to use it by updating its name in the script

## 🧪 Usage

```bash
./screen_recorder.sh your-project-name
```


The script uses the following path logic (edit this in the script as needed):

By default:
- Recordings are saved under `/Volumes/T7/preferred/path/<project_name>`
  - `T7` is the name of my SSD drive
  - `preferred/path` is a path within this drive
  - `<project_name>` is the name of the project provided on the command line.
  The corresponding folder **must** exist before the script runs.
  - Each project gets its own folder
  - Each session gets stored in `0/`, `1/`, `2/`, etc. **Automatically**

> :bulb: You can change this to any location on your machine — just update the `RECORDINGS_PATH` variable in the script.

> :warning: This script doesn’t currently check available storage — if your drive fills up
mid-session, the recording will fail silently. Be mindful of free space, especially 
when recording long sessions.


1. Run the script with your project name
2. Press any key to start recording
3. Press any key to stop recording
4. `current/` is automatically archived to a numbered folder

## 📁 Directory Requirements

- If `current/` is not empty, the script will prompt you to continue or archive its contents into `fail/`

## 💡 Why it matters

Because recording shouldn’t be a burden.
Because **clicking record is slower than scripting it**.
Because every dev deserves to document their journey — without breaking flow.

## 🧱 Status

This is an early, raw version. But it works.
Use it, fork it, break it, improve it.

> Built by [Shelltief](https://shelltief.sh) to document real work, in a smooth way.
