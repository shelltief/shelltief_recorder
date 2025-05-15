# shelltief-recorder

A dead-simple Bash script to record your screen, webcam, and mic simultaneously — all from the terminal.

Built for:
- Developers who build and want to **document their process**
- Builders who prefer **terminal rituals over clicking buttons**
- People who want to record **directly to disk** with zero overhead

## ⚙️ Features

- Records your **screen** and **mic** (macOS only)
- Detects and records your **webcam** (FaceTime or iPhone via Continuity Camera)
- Saves to a `current/` directory, then auto-archives to `0001/`, `0002/`, ...
- Manages process PIDs for clean starts and stops
- Fully terminal-driven — no GUI, no popups, no bullshit

## 🍏 macOS Only (for now)

This script uses Apple's `avfoundation` backend and expects `ffmpeg` installed (via Homebrew or similar).

To list available input devices:
```bash
ffmpeg -f avfoundation -list_devices true -i ""
```

By default:

* `"FaceTime"` is used as the fallback webcam
* `Nokia` is expected to be your iPhone — you’ll likely need to edit this name in the script.
(For example, my iPhone is named `Nokia de Thibault` so `Nokia` matches it.)

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
- Recordings are saved under `/Volumes/T7/code_videos/Rushes`
  - `T7` is the name of my SSD drive
  - Each project gets its own folder
  - Each session gets stored in `0/`, `1/`, `2/`, etc. **Automatically**

> :bulb: You can change this to any location on your machine — just update the `RECORDINGS_PATH` variable in the script.

> :warning: This script doesn’t currently check available storage — if your drive fills up
mid-session, the recording will fail silently. Be mindful of free space, especially 
when recording long sessions.

* Starts recording screen + mic + cam
* Waits for user input to stop (press 3 random keys)
* Moves `current/` to an archived folder

## 💡 Why it matters

Because recording shouldn’t be a burden.
Because **clicking record is slower than scripting it**.
Because every dev deserves to document their journey — without breaking flow.

## 🧱 Status

This is an early, raw version. But it works.
Use it, fork it, break it, improve it.

> Built by [Shelltief](https://shelltief.sh) to document real work, in a smooth way.
