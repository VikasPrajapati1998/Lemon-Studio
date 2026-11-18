# Lemon Studio user manual

> **About this manual.** It describes the pre-alpha build of Lemon Studio as found in this repository, and it may lag
> behind it. Menu names, shortcuts and features change from one build to the next. Where this manual and the program
> disagree, the program is right. For the latest guides, see the [wiki](https://getlemonstudio.app/wiki/).
>
> Entries marked **(unverified)** could not be fully confirmed from the source and documentation.

## Contents

1. [Introduction and status](#1-introduction-and-status)
2. [Installing and starting](#2-installing-and-starting)
3. [First launch and interface tour](#3-first-launch-and-interface-tour)
4. [Making a beat in the instrument rack](#4-making-a-beat-in-the-instrument-rack)
5. [Piano roll](#5-piano-roll)
6. [Arranger and performance mode](#6-arranger-and-performance-mode)
7. [Mixer, routing, effects, sidechain and delay compensation](#7-mixer-routing-effects-sidechain-and-delay-compensation)
8. [Plugins](#8-plugins)
9. [Automation](#9-automation)
10. [Recording and the Tesla audio editor](#10-recording-and-the-tesla-audio-editor)
11. [Time stretching](#11-time-stretching)
12. [Exporting audio and MIDI](#12-exporting-audio-and-midi)
13. [MIDI controllers and MIDI learn](#13-midi-controllers-and-midi-learn)
14. [Project files](#14-project-files)
15. [Settings and themes](#15-settings-and-themes)
16. [Keyboard shortcuts](#16-keyboard-shortcuts)
17. [Logs and troubleshooting](#17-logs-and-troubleshooting)
18. [Scripting and assistants](#18-scripting-and-assistants)
19. [Known limitations](#19-known-limitations)
20. [Glossary](#20-glossary)

---

## 1. Introduction and status

Lemon Studio is a free, open-source (GPL-3.0-or-later), pattern-based digital audio workstation for Windows, Linux and
macOS. The basic workflow is:

1. Build **patterns** in the **instrument rack** (step sequencer) and the **piano roll**.
2. Lay the patterns, audio clips and automation clips out as a song in the **arranger**.
3. Balance and process the sound in the **mixer**.
4. **Export** the result to audio or MIDI files.

It comes with 29 built-in instruments and effects, and it hosts CLAP, VST3 and VST2 plugins.

> **Warning: pre-alpha software.** Features are missing, and things can change in ways that break your work.
> - **Back up anything you care about.** Work on copies of your projects, especially `.flp` files.
> - Greyed-out controls are not implemented yet. Hovering over one shows "not available yet". Clicking it does nothing.
> - A few controls look like they work but change nothing you can hear (see [Known limitations](#19-known-limitations)).

Support is not offered at this stage. You can follow development on the project's Discord server and report bugs on
GitHub (see [section 17](#17-logs-and-troubleshooting)).

---

## 2. Installing and starting

### 2.1 Supported systems

| System | Requirements | Audio drivers |
|---|---|---|
| Windows | Windows 10 or 11, x64 | WASAPI (shared or exclusive mode), ASIO |
| Linux | x64, on X11, or on Wayland through XWayland | PulseAudio or PipeWire (through pipewire-pulse), ALSA, JACK (not yet tried against a running JACK server) |
| macOS | macOS 11 or newer (downloads will be for Apple silicon) | Core Audio |

### 2.2 Downloads

**No builds have been released yet.** The first ones will appear on the
[download page](https://getlemonstudio.app/download/). Release builds will update themselves. They are not signed, so
your system may block them the first time you run them. The
[install guide](https://getlemonstudio.app/wiki/getting-started/install/) explains how to open them.

Until then, the only way to run Lemon Studio is to build it from source.

### 2.3 Building from source

You need:

- [Rust](https://rustup.rs/), stable channel, version 1.89 or newer.
- Git.
- A few system packages. The [build guide](https://getlemonstudio.app/wiki/reference/build-from-source/) lists them for
  each system. For reference, the project's CI installs these on Ubuntu:
  `libasound2-dev libjack-jackd2-dev libwayland-dev libclang-dev pkg-config`. Windows and macOS also need libclang
  (CI uses the LLVM that ships with the Windows build image and Xcode's on macOS).

Steps (the same on all three systems):

1. Open a terminal.
2. Download the source:
   ```bash
   git clone https://github.com/ItsAlphaNeon/Lemon-Studio.git
   cd Lemon-Studio
   ```
3. Build and start Lemon Studio:
   ```bash
   cargo run -p os-app --release
   ```

The first build takes a long time. Before Lemon Studio starts, the build also compiles Lemon Studio's own plugins and
installs them as VST3 and CLAP plugins, in the folders where other DAWs find them. Lemon Studio itself does not need
those files: every built-in plugin is also compiled into the program. Later runs rebuild only what changed.

A copy built from source never updates itself. To update it, run `git pull` and build again.

On Windows, `lemonstudio.exe` is a GUI program. Started from Explorer or a shortcut, it opens no console window.

### 2.4 Command-line options

When you run through Cargo, put the program's options after `--`, for example
`cargo run -p os-app --release -- song.lemon`. If you run the program directly, it is called `lemonstudio`.

| Command | What it does |
|---|---|
| `lemonstudio path/to/song.lemon` | Opens a project at startup (a `.flp` works too). |
| `lemonstudio --render in.lemon out.wav` | Renders the song to a 32-bit float WAV without opening a window or an audio device. |
| `--rate HZ` (with `--render`) | Render sample rate, 8000 to 384000. The default is 44100. |
| `--seconds S` (with `--render`) | Stops after this many seconds of song. |
| `--from TICK` (with `--render`) | Starts at this tick. |
| `--builtin-only` (with `--render`) | Hosts only Lemon Studio's own plugins, so the render is the same on every machine. Without it, the plugins found by the last plugin scan are used too. |
| `--control <socket>` | Starts the control plane beside the window so scripts and assistants can drive the app. See [section 18](#18-scripting-and-assistants). |
| `--serve <socket> [project]` | Runs the control plane with no window. |
| `--control-level read\|propose\|edit` | Limits what control-plane clients may do. The default is `edit`. |
| `--control-token <token>` | The token clients must give when connecting. |

On Windows, if you start `--render` directly from PowerShell, add `| Out-Default` (or use `start /wait` in `cmd`) so
the terminal waits until the render is done.

### 2.5 Environment variables

| Variable | Effect |
|---|---|
| `OPENSTUDIO_USER_DIR` | Uses another user folder for settings, themes, presets and logs (see [17.1](#171-the-user-folder)). |
| `OPENSTUDIO_NO_PLUGIN_BUILD=1` | Skips building and installing the built-in plugins when you start with `cargo run`. |
| `OPENSTUDIO_LOG_DIR` | Moves the application log folder (it does not move the per-project logs). |
| `OPENSTUDIO_LOG` | `debug`, `info`, `warn` or `error`: the least important level written to the log. The default is `info`. |
| `OPENSTUDIO_WAYLAND=1` | Linux only: asks for native Wayland even when an X server is available. |
| `OPENSTUDIO_FACTORY` | macOS only: where FL Studio's factory content is looked for. By default, the newest `/Applications/FL Studio *.app`. |
| `CLAP_PATH`, `VST_PATH` | Extra plugin folders (see [8.3](#83-plugin-folders)). |

---

## 3. First launch and interface tour

### 3.1 The main window

The top of the main window holds:

- **The menu bar:** FILE, EDIT, ADD, PATTERNS, VIEW, OPTIONS, TOOLS and HELP.
- **The toolbar**, with these controls:
  - The **PAT / SONG switch.** "Pattern" plays the current pattern; "Song" plays the arrangement. The **L** key flips it.
  - **Play / pause and Stop.**
  - The **Record** button.
  - The **tempo** box. Drag it up or down to change the whole number, or the decimal place where your drag starts.
  - The **metronome** button.
  - The **song position** slider.
  - The **CPU panel** and the **oscilloscope / spectrum** display (click it to switch views).
  - The **output meter** and the **main volume**.
  - Buttons for the windows and for common commands.
- **The hint bar.** It describes the control under the mouse pointer and shows the result of a command. When you're
  unsure what something does, hover over it and read the hint bar.

Hovering over any toolbar button shows its name and shortcut. Right-click the toolbar to change its layout and presets.

### 3.2 The windows

You can open and close each window from the VIEW menu, its toolbar button or its shortcut key.

| Window | Shortcut | Purpose |
|---|---|---|
| Arranger | F5 | The song: pattern, audio and automation clips on tracks |
| Instrument rack | F6 | Channels (instruments) with step sequencers |
| Piano roll | F7 | Note editing for one channel |
| Mixer | F9 | Mixer tracks, effects and routing |
| Browser | Alt+F8 | Files, plugins, presets, the project's contents and the undo history |
| Plugin palette | F8 | Quick list of instruments and effects to add |
| Clip palette | Ctrl+F8 | Menu of the project's patterns and channels |
| Tap tempo | Ctrl+F5 | Tap to set the tempo |
| Touch controller | Alt+F7 | On-screen 4 x 4 pads or two-octave keyboard |
| Plugin performance monitor | (double-click the CPU panel) | Per-plugin processing time |

You can change which F8 combination opens the browser and the palettes in
**General settings ▸ Input ▸ Manage shortcuts**. The menus always show the key that is in effect.

**Window layout:**

- **VIEW ▸ Arrange windows** puts the windows back in a standard layout. It offers Realign (Shift+Ctrl+H), Desktop
  (default), Desktop (alternate), Save custom layout and Custom. The Tablet layouts are greyed out.
- **VIEW ▸ Close all windows** (F12), **Close all plugin windows** (Alt+F12) and **Close all unfocused windows**
  (Ctrl+F12) tidy up. **Align all channel editors** (Shift+F12) lines up the plugin windows.
- **VIEW ▸ Background ▸ Set image wallpaper...** uses a PNG or JPEG picture as the workspace wallpaper.
- The mixer and the instrument rack can be **Detached** into windows of their own (in their options menus), for
  example to move them to another screen.
- Lemon Studio remembers where the windows were when it closed and puts them back at the next start.

### 3.3 Transport basics

| Action | How |
|---|---|
| Play / stop (rewinds) | Space |
| Play / pause | Ctrl+Space, or the Play button |
| Switch between pattern and song | L, or the PAT / SONG switch |
| Play the song from the start | Ctrl+Enter with the arranger focused |
| Stop all sound, including effect tails | TOOLS ▸ Stop sound (Ctrl+H) |
| Metronome on / off | Ctrl+M, or OPTIONS ▸ Metronome. Right-click the toolbar button for five sounds and the volume (-12 to +12 dB). |

The metronome is never included in an export, and Lemon Studio remembers its setting between sessions.

**Tap tempo:** open it with VIEW ▸ Tap tempo (Ctrl+F5).

1. Click the pad in time with the music. You can also tap a key on a MIDI keyboard, a note on the typing keyboard or a
   Touch controller pad.
2. While **Apply tempo** is on, the average of your taps (in whole BPM) becomes the project tempo.

The nudge buttons play 2 % slower or faster while you hold them. **Sync**, **Mute** and **now** are not available yet.

### 3.4 Playing notes

You can play notes three ways. Notes always play the selected channel.

- **The computer keyboard.** Turn it on with OPTIONS ▸ Computer MIDI keyboard (Ctrl+T) or the toolbar button. While it
  is on, letter keys play notes, and numpad + / − change the octave. Some single-letter shortcuts (such as tool letters
  and R for record) are taken as notes while it is on.
- **A MIDI keyboard.** Enable it in Settings ▸ MIDI.
- **The Touch controller** (Alt+F7).

---

## 4. Making a beat in the instrument rack

The instrument rack (F6) lists the project's **channels**. Each channel is one instrument with its own row of steps.

### 4.1 Adding channels

- **ADD ▸ Sampler** adds a Lemon Sampler channel.
- **ADD ▸** an instrument from the plugin menus adds that instrument. Choose how the menus are organized with the
  Categories / Simple / By vendor options at the bottom of ADD.
- The **Plugin palette** (F8) lists instruments and effects. An instrument goes to the bottom of the rack.
- You can also drag a sample or plugin from the browser into the rack.
- **ADD ▸ Layer** makes a layer channel whose children are the channels selected in the rack.
- **ADD ▸ Audio clip** and **ADD ▸ Automation clip** add those channel types. An automation clip made this way can't be
  linked to a target yet (see [section 9](#9-automation)).
- **ADD ▸ Lemon MIDI Out** adds a channel that sends notes to an external MIDI device (see
  [section 13](#13-midi-controllers-and-midi-learn)).

### 4.2 Programming steps

1. Choose or create a pattern. Use the PATTERNS menu, or **Next empty pattern** (F4).
2. In a channel's row, click steps to switch them on. Left-click or drag paints steps on; right-click or drag removes
   them.
3. Press Space to hear the pattern loop.
4. Set the **Pattern length** with the box at the top of the rack.
5. Use the **swing** control, and the channel's swing settings, to shuffle the groove.

### 4.3 Channel controls

- **Name button:**
  - Click it to open the channel's plugin window.
  - Shift+click it to rename the channel.
  - Right-click it for the channel menu, which includes **Change icon...**, **Show waveform preview**, **Receive notes
    from ▸**, **Burn MIDI to ▸**, **Wrap in container** and **Assign to new instrument track**.
- **Mute / solo light:** click it, or press 1 to 0 to mute channels 1 to 10. Ctrl plus a digit solos a channel.
- **Mixer track box:** sets which mixer track the channel plays into. Double-click it, or press Ctrl+L to route the
  selected channels to a free mixer track.
- **Piano roll preview:** channels that have piano-roll notes show a mini preview in place of steps. Click it to open
  the piano roll.
- **Step parameters** (Ctrl+K): the graph editor for step values. Its Ctrl, right-button and Alt gestures are not
  available yet.

**Selecting, moving and filtering channels:**

- Up / Down move the selection. Alt+Up / Down move the selected channels.
- The display filter (PgUp / PgDn) shows one channel group at a time.
  > **Note:** a new or cloned channel joins no group, so it stays hidden while a group filter is on.

### 4.4 Per-channel loops

Each row can loop at its own length:

1. Open the rack's **Loop mode** button.
2. Choose **Advanced looping**.
3. Use each row's loop box to set its loop point (Step, Beat or Bar).

The rack's menu also offers **Burn selected loops to pattern**.

### 4.5 Rendering a pattern to audio

- **PATTERNS ▸ Render to audio clip** renders the current pattern to a WAV in the data folder's `Rendered` folder and
  makes a new audio clip channel from it.
- **Render as audio clip...** first opens the Export form so you can choose the format and options.
- **Render and replace...** also replaces the pattern's clips in the arranger with the audio and mutes the pattern clips.

### 4.6 Not available yet in the rack

The following do nothing yet:

- The Misc tab's arpeggiator, echo delay, gate, time shift, tracking and Full porta (greyed out).
- A layer channel's settings beyond volume and pan.
- Loop starter, Auto fill and Create instrument.
- Filter groups for patterns.
- **Loop step channels** and **Loop all channels**: these look live but change nothing heard.

---

## 5. Piano roll

The piano roll (F7) edits the notes of the selected channel in the current pattern.

### 5.1 Tools

Choose a tool on the toolbar or by its letter key. With the typing keyboard on, letters play notes, so use the toolbar
instead.

| Tool | Key | What it does |
|---|---|---|
| Draw | P | Click to place notes. Drag to move them; drag their edges to resize them. |
| Paint | B | Drag to paint notes along a row. |
| Delete | D | Click or swipe to delete notes. |
| Mute | T | Click or swipe to mute or unmute notes. |
| Slice | C | Drag a line to cut notes. |
| Select | E | Drag a box to select notes. Ctrl adds to the selection. |
| Zoom | Z | Drag a box to zoom. Right-click zooms out or back. |
| Playback | Y | Drag to hear the notes under the line. |

Turning the mouse wheel while holding the right button picks the next tool.

The toolbar's **Piano roll options**, **Note properties**, **Scale and chord helpers** and **Paint and add** buttons are
greyed out. They are not available yet.

### 5.2 Editing

- **Copy, cut and paste:** Ctrl+C, Ctrl+X and Ctrl+V.
- **Select all:** Ctrl+A. This also makes the whole pattern the time selection.
- **Deselect:** Ctrl+D.
- **Delete:** the Delete key removes the selected notes. With nothing selected, it removes every note of the channel.
- **Move notes:**
  - Shift+arrows move them by a snap step or a semitone.
  - Ctrl+Up / Down moves them an octave.
  - Alt+Left / Right moves them by one tick.
  - Ctrl+Shift+Left / Right rotates them.
- **Scroll:** Left / Right on their own scroll by one bar.
- **Zoom:** PgUp / PgDn zoom in time.
- **Snap:** the magnet button chooses the snap and the length of new notes. Backspace turns the global snap on and off.
- **Selection keys:** Shift+I inverts the selection, Shift+R selects a random note, Shift+M selects about half of the
  notes at random, and Shift+C selects the notes of the current colour.
- **Time selection:** Ctrl+Enter makes the time selection span the selected notes.
- **Changing channels:** Shift+K (or Ctrl+K) goes to the next channel, and Shift+H to the next channel that has notes.
  With the typing keyboard off, G / K go to the previous / next channel, and H / J to the previous / next channel with
  notes.
- **Event panel:** the velocity and event lanes sit below the notes. Drag the splitter to resize the panel, or
  double-click the splitter to hide or show it.

### 5.3 The Tools menu

Open it with the toolbar's tools button. Every entry is also on the keyboard.

| Entry | Key |
|---|---|
| Import MIDI file... / Export as MIDI file... | (none) |
| Duplicate | Ctrl+B |
| Glue | Ctrl+G |
| Discard lengths | Shift+D |
| Quick quantize | Ctrl+Q |
| Quick quantize start times | Shift+Q |
| Quantize... | Alt+Q |
| Quick legato | Ctrl+L |
| Quick chop | Ctrl+U |
| Chop... | Alt+U |
| Strum... | Alt+S |
| Arpeggiate... | Alt+A |
| Flam... | Alt+F |
| Claw machine... | Alt+W |
| Limit... | Alt+K |
| Flip... | Alt+Y |
| Randomize... | Alt+R |
| Scale levels... | Alt+X |
| LFO... | Alt+O |
| Ghost channels | Alt+V |
| Add time marker | Alt+T |

Alt+M mutes the selected notes, and Alt+Shift+M unmutes them.

### 5.4 Step edit

Step edit (OPTIONS ▸ Step edit, Ctrl+E, or its toolbar button) works while the transport is stopped:

1. Turn step edit on.
2. Play notes on the typing keyboard or a MIDI keyboard. Each chord you play is written at the song position, one snap
   step long, and the position moves on by one step.

### 5.5 Chord progression tool

**TOOLS ▸ Chord progression tool...** writes a progression into the piano roll's pattern, one chord per bar, after the
pattern's last note. You can type it two ways:

- As chord names, for example `C G Am F7 Dm/F`.
- As numerals, for example `I V vi IV`. Put the key first, for example `Am:`.

### 5.6 Not available yet in the piano roll

- Note colours.
- Scale and chord helpers, Paint drum, Riff machine, Articulate and scripts.
- Toggles for slide and portamento notes.
- A sound for the **Release** lane: the lane is there, but nothing uses release velocity.

---

## 6. Arranger and performance mode

### 6.1 Placing clips

The arranger (F5) holds **pattern clips**, **audio clips** and **automation clips** on tracks.

1. Choose the clip source in the Clip palette (Ctrl+F8) or the browser.
2. Use the Draw tool (P) and click a track to place the clip.
3. Drag clips to move them, and drag their edges to resize them.

| Tool | Key | What it does |
|---|---|---|
| Draw | P | Click places the current clip. Drag moves; edges resize; right-click deletes. |
| Paint | B | Drag paints clips along the track. |
| Delete | D | Click or drag deletes clips. |
| Mute | T | Click or drag mutes or unmutes clips. |
| Slip | S | Drag moves a clip's content inside it. |
| Slice | C | Drag a line to cut clips. |
| Select | E | Drag a box to select. Ctrl adds; Shift toggles; right-click clears. |
| Zoom | Z | Drag a box to zoom. Right-click toggles. |
| Playback | Y | Hold to play from the mouse. |

### 6.2 Editing keys (arranger focused)

| Key | Action |
|---|---|
| Ctrl+A / Ctrl+D | Select all / deselect |
| Ctrl+C / Ctrl+X / Ctrl+V | Copy / cut / paste |
| Ctrl+B | Duplicate |
| Delete | Delete the selection |
| Ctrl+Insert / Ctrl+Delete | Insert space / delete space |
| Shift+arrows | Move the selection in time or across tracks |
| Alt+M / Alt+Shift+M | Mute / unmute |
| Shift+G / Alt+G (or Alt+U) | Group / ungroup clips |
| Shift+I | Invert the selection |
| Shift+Ctrl+C | Make unique |
| Ctrl+Alt+C | Consolidate (see the limitation below) |
| Alt+T / Alt+Shift+T | Add a time marker / a time signature marker |
| Shift+T | Place loop |
| Shift+M | Arranger stretch mode |
| Shift+Tab | Cycle the snap |
| Backspace | Global snap on / off |
| PgUp / PgDn | Zoom in / out |
| Shift+5, Shift+Z or End | Zoom on the selection |
| Shift+4 | Zoom out to everything |
| Ctrl+Up / Ctrl+Down | Select the previous / next track |
| Ctrl+Enter | Play the song from the start |
| Ctrl+P | Performance mode on / off |

**Grouped clips move together** (OPTIONS menu, Shift+Alt+G) decides whether grouped clips select and move together.
Alt+click a track's mute switch to mute its whole group; Alt+right-click to solo it.

### 6.3 Markers, loops and arrangements

- **Time markers:** right-click a marker for its actions: Start, Loop, Marker loop, Marker skip, Marker pause, Stop
  recording and Start recording. Exports begin at a Start marker and jump at skips.
- **Time selection:** a time selection in the arranger (SONG mode) loops playback.
- **Key markers and time signature markers** can also be added.
- **Several arrangements** can be kept in one project.
- **Auto scrolling** (OPTIONS menu or the toolbar) makes the view follow the playhead.
- **Track height, zoom and scroll** are saved with the project.
- **Audio tracks and instrument tracks:** link an arranger track to a mixer track or a channel. Use the track menu's
  Track mode ▸, or the mixer track menu's **Assign to new Audio track(s)**.
- **TOOLS ▸ Lock all tracks to their clips** makes Draw and Paint place each track's own source on it.
  **Unlock all tracks from their clips** clears the locks.

> **Bars and time signatures:** `docs/status.md` and the code's tests say the arranger's bars, ruler and Bar snap
> follow the project's time signature and time signature markers. The README and the alpha checklist still say bars are
> always 4 beats. Check your build (unverified).

### 6.4 Performance mode

Performance mode turns the start of the arranger into a clip launcher.

1. Choose **Prepare for performance mode** (in the arranger's ▸ menu) or **TOOLS ▸ Set up for live performance**.
   After asking first, Lemon Studio lays out the launch columns and places a Start marker.
2. Turn on **Performance mode** (Ctrl+P with the arranger focused, or the arranger's ▸ menu).
3. Click a clip before the Start marker to launch it, looping, from the next sync point:
   - Shift+click queues the clip.
   - Alt+click launches it at once.
   - Right-click the clip, or click the track header, to stop it.
   - Ctrl+click launches a whole scene.
4. To record a performance, arm Record. When you stop, what the launched tracks played after the Start marker is written
   as clips.

You can launch clips from other sources too:

- **MIDI:** notes 0–119 on the **Performance mode MIDI channel** (set in Settings ▸ MIDI) launch the first 10 tracks'
  12 columns. Notes 120–124 scroll the grid, 125 and 126 trigger scenes, and holding 127 syncs launches to the global
  snap.
- **Typing keyboard:** in performance mode, the rows 1–5, Q–T, A–G and Z–B launch tracks 1–4, and 6–0, Y–P, H–; and
  N–/ launch tracks 5–8, five columns each.

Each track's **Performance settings ▸** (Press, Motion, Trigger sync, Queue, Tolerant, Position sync) are saved with the
project.

### 6.5 Not available yet in the arranger

- A clip properties window, and per-clip pitch, pan and reverse.
- Chop and merge.
- Make unique for automation clips.
- The fade and gain right-click menus.
- Per-track height.
- **Consolidate** and **Export all playlist tracks** render without hosted instruments and effects, so plugin channels
  come out silent. Consolidate still mutes the source clips.

---

## 7. Mixer, routing, effects, sidechain and delay compensation

### 7.1 Mixer tracks

Open the mixer with F9. Each strip has a fader, pan, mute, solo, meters and up to 10 effect slots. The master track is
on the left.

**Selecting and moving tracks:**

- Left / Right select the neighbouring track.
- Alt+Left / Alt+Right move the track.
- To select several tracks, Ctrl+click them, or Ctrl+Shift+drag across the strips. Moving one fader then moves all the
  selected faders by the same number of dB.
- Shift+wheel over a strip moves its track.

**Other mixer keys:** S solos the track, Alt+S alt-solos it, F2 renames it, and Delete removes it.

**Track menu:** right-click a track for its menu. It includes:

- Rename, color and icon..., Change color..., Random color, Gradient color... (with several tracks selected) and
  Change icon...
- File ▸ Open mixer track state / Save mixer track state (saved as `.lemontrack` files in the user folder's Mixer states
  folder) / Render to wave file...
- Open audio editor and Open audio editor and record.
- Channel routing, Track routing, Select ▸, Reset selected track(s) routing.
- Solo, Alt solo, Arm and Disarm selected track(s).
- The Group section: Separator, Create group... and Auto color group.
- Dock to ▸ Left / Middle / Right, Move left / right, Delete, Insert one, Insert multiple..., Insert from state.
- Allow threaded processing.

> **Note:** the track menu shows Ctrl+E and Shift+E beside the audio editor entries, but those keys do something else
> (Ctrl+E is Step edit). Use the menu entries instead.

**Mixer options ▸ View:**

- **Waveforms** (Alt+W) draws each track's recent peaks as a scrolling waveform in place of its meter.
- **Extra properties** shows or hides the polarity, swap and stereo-separation controls.

### 7.2 Routing

Any track can be routed to any other.

- **Route channels:** select channels in the rack, then use the track menu's Channel routing ▸ **Route selected
  channels to this track** (Ctrl+L) or **...starting from this track** (Shift+Ctrl+L).
- **Route tracks:** **Track routing ▸** routes or sidechains the other selected tracks into the track you clicked.
- **Find routed items:** **Select ▸ Channels routed to this track** (Alt+L) and **Tracks routed to this one**
  (Ctrl+Alt+L).
- **Outputs:** with an ASIO driver, every track has an output menu with the pairs of the driver's first 16 outputs.

### 7.3 Effects

1. Select a mixer track.
2. Add an effect with the Plugin palette (F8, which puts effects on the selected mixer track), ADD ▸ Effect, or an empty
   slot's menu.
3. Use the slot's switch and mix knob to bypass it or blend it.

Right-click a slot for its menu:

- Presets, **Browse presets** and **Save preset as...**
- **Sidechain input**.
- **Scaling ▸** (100 % to 200 % for hosted plugin windows).
- **Help**, Change color..., Change icon... and **Link all parameters...**
- **Wrap in container** and **Wrap all in container**.

**Containers** hold several effects in one slot. They play the effects in series, each with its own wet and dry levels.
Open a container's list with the slot menu's **Container ▸ Edit list...** to reorder, add or remove effects. Free routing
between the plugins inside a container is not available yet. An instrument channel can also be wrapped in a container
(the rack menu's Wrap in container) to layer several instruments.

### 7.4 Sidechain

- **Built-in effects:** a Lemon Limiter in a slot listens to the track routed into its track that its sidechain number
  names.
- **CLAP and VST3 effects:** choose the input in the slot menu's **Sidechain input** submenu. It lists the tracks routed
  into this track. Routing a track to another with level 0 makes a sidechain-only route.
- **Lemon Vocoder** takes its voice or carrier through the sidechain input.

### 7.5 Plugin delay compensation

Lemon Studio makes up the delay each plugin reports, on every path through the mixer, so everything stays in time.
Exports start late by the whole delay, so the file has none and keeps its length.

- **Mixer options ▸ Plugin delay compensation ▸ Automatic** (on by default): when you untick it, plugin delays are no
  longer compensated. The tracks' own latency settings still apply.
- To set a track's latency, right-click the strip's latency clock or use the input clock. Choose Reset, or type a value
  in ms, samples or beats. The clocks turn orange when set.

**Not available yet:** the layout combo's other mixer layouts, and the latency menus' Adjust position of recorded audio,
Set from, Delay audio output, Bypass track latency compensation and Compensate for master latency.

---

## 8. Plugins

### 8.1 Built-in plugins

Lemon Studio has 29 built-in plugins. All but Lemon MIDI Out and Lemon Container are also installed as free VST3 and CLAP
plugins for other DAWs.

| Kind | Plugins |
|---|---|
| Instruments | Lemon Sampler, Lemon Trio (three-oscillator synth), Citrus (six-operator synth), Smith (additive synth), Lemon Slicer (loop slicer), Lemon Drum Sampler (4 x 4 pads) |
| Effects | Lemon Parametric EQ, Lemon Limiter (limiter, compressor, gate), Lemon Reverb, Lemon Delay, Lemon Chorus, Lemon Flanger, Lemon Phaser, Lemon Filter, Lemon Vocoder, Lemon Soft Clipper, Lemon Fast Distortion, Lemon Transient Processor, Lemon Balance, Lemon Stereo Enhancer, Lemon Stereo Shaper, Pulp Filter (multi-filter), Press (multiband maximiser), Lemon Drop (audio visualizer) |
| Controllers | Lemon Peak Controller, Lemon Formula Controller |
| Tools | Tesla (audio editor and recorder), Lemon MIDI Out, Lemon Container |

Some settings of the larger plugins are stored but not played yet. For example:

- Lemon Slicer's time stretch.
- Lemon Drum Sampler's pad envelopes.
- Lemon Vocoder's built-in synth carrier.
- Some of Lemon Parametric EQ's options, which are greyed out.

The [catalogue](https://getlemonstudio.app/instruments-and-effects/) describes each plugin. A built-in plugin's Help
opens its page in the wiki.

### 8.2 Loading CLAP, VST3 and VST2 plugins

- Lemon Studio scans for plugins in the background at startup. Each file is tried in a separate scanner process, so a
  broken plugin can't crash Lemon Studio.
- **ADD ▸ Rescan plugins (fast)** scans again.
- Plugins appear in the ADD menu, the Plugin palette (F8), the mixer slot menus and the browser's **Plugin library**
  (Shift+F8).
- A plugin installed in several formats shows once; your preferred format wins.
- **VST2 shells** (one file offering several plugins) and **multi-output instruments** are supported. To route a
  multi-output instrument, use the plugin window menu's **Outputs ▸**, or **Auto map outputs**.
- **Crash protection:** a plugin that crashes is stopped and its slot passes the sound through, instead of Lemon Studio
  closing. On macOS, a crash in a plugin's *editor* still closes Lemon Studio.
- **Not available yet:** 32-bit plugins, Audio Units, Intel-only plugins on Apple silicon, and effects' outputs beyond
  their main stereo pair.

### 8.3 Plugin folders

Lemon Studio looks in these folders, plus any you add in Settings ▸ Plugins:

| System | Folders |
|---|---|
| Windows | `%LOCALAPPDATA%\Programs\Common\VST3` and `\CLAP`, `%COMMONPROGRAMFILES%\VST3` and `\CLAP`, and `CLAP_PATH`. VST2: the folder installers record in the registry (`HKEY_LOCAL_MACHINE\SOFTWARE\VST`, `VSTPluginsPath`), plus whichever of `%PROGRAMFILES%\VSTPlugins`, `%PROGRAMFILES%\Steinberg\VSTPlugins`, `%COMMONPROGRAMFILES%\VST2` and `%COMMONPROGRAMFILES%\Steinberg\VST2` exist |
| Linux | `~/.clap`, `/usr/lib/clap`, `/usr/local/lib/clap` and `CLAP_PATH`; `~/.vst3`, `/usr/lib/vst3`, `/usr/local/lib/vst3`. VST2: `VST_PATH`, else `~/.vst`, `/usr/lib/vst`, `/usr/local/lib/vst` and the `lxvst` folders that exist |
| macOS | `~/Library/Audio/Plug-Ins/CLAP`, `VST3` and `VST`, `/Library/Audio/Plug-Ins/CLAP`, `VST3` and `VST`, and `CLAP_PATH` |

### 8.4 The plugin manager

Open it with **OPTIONS ▸ Manage plugins** or **ADD ▸ Manage plugins** (Settings ▸ Plugins). It has:

- Your own search folders, which you can add, switch off and remove.
- A preferred-format choice.
- A plugin table that you can sort and filter with **Find**. Each row's hint shows the file path, or why the file failed.
- A star for favourites and a switch to hide a plugin.

### 8.5 Plugin windows and presets

- **Presets:** the title bar's preset box lists your own presets, then the factory presets. Step through them with the
  arrows or the mouse wheel. The window menu has **Save preset as...**, Rename, Delete and Browse presets.
- **Importing presets:** `.ospreset`, `.vstpreset` and VST2 `.fxp` / `.fxb` files import. Drop a preset file onto a
  plugin window, rack row or mixer slot to load it there.
- **A/B compare:** **Copy to B**, then **Swap A / B**.
- **Window menu:** it also has **Link all parameters...**, **Show all parameters**, Allow threaded processing,
  **Auto-bypass when silent**, **Save channel state as...**, **Browse channel states** and Detach / Attach.
- **Auto-bypass for all plugins:** **TOOLS ▸ Switch auto-bypass for all plugins** turns Auto-bypass when silent on or off
  for every plugin at once.
- **Keyboard focus:** **TOOLS ▸ Set keyboard focus of all plugins to ▸ Disabled** keeps plugin editors from taking typed
  keys (Windows and macOS).

---

## 9. Automation

### 9.1 Automation clips

1. Right-click a control: a channel knob, a mixer control, or a plugin parameter (in the browser after **Show all
   parameters**, or in the plugin's own right-click menu for CLAP and VST3 plugins).
2. Choose **Create automation clip**. A clip channel appears in the rack's "Automation" group, and the clip appears in
   the arranger.
3. Draw the curve in the clip. All thirteen curve shapes are played.
4. Open the clip's channel window to set its range (the MIN and MAX knobs) and its LFO.

**ADD ▸ Automation for the most recent parameter** and **TOOLS ▸ Recent parameters** act on the last control you
touched.

### 9.2 The event editor

Choose **Edit automation** on a control's menu to edit the control's events in the current pattern.

- **Drawing:** left-drag draws freehand on the snap, and right-drag draws a straight line. The tools are Paint, Erase,
  Interpolate, Select and Zoom (Z).
- **Edit menu:** Cut, Copy, Paste, Select all, Select previous / next (Ctrl+Left / Right), Delete, Duplicate (Ctrl+B),
  Insert space (Ctrl+Ins), Delete space (Ctrl+Del), Zoom in / out (PgUp / PgDn), Zoom on selection (Shift+5) and **Turn
  into automation clip**.
- **Tools:** Scale levels and LFO (Ctrl+L).
- **Edit automation in a window** opens a second event editor on another control.

### 9.3 Recording automation

While you record, moves of the following controls become the current pattern's control points:

- A channel's volume and pan.
- A mixer track's fader, pan, stereo separation and equaliser.
- The slot mix knobs.
- The tempo.
- Any plugin parameter moved in its own window.

Send levels, the swing knob and the pitch knobs are not recorded yet.

### 9.4 Not available yet

- Editing an automation clip's targets. A clip made with ADD ▸ Automation clip can't be linked to anything, so it drives
  nothing.
- Automation entries on stereo separation, slot mix, sends, main volume and main pitch.
- Making controller links (Peak / Formula Controller links) inside the app. Links already in a project do play.
- In the event editor: Auto smoothing, its File, View, Helpers, Snap, Target and Time markers menus, and a detached
  window.

---

## 10. Recording and the Tesla audio editor

### 10.1 Recording notes

1. Click **Record** (or press R while the typing keyboard is off).
2. Choose what to record: **Notes and automation**, **Notes only**, **Automation only**, **Audio, into the arranger as an
   audio clip** or **Audio, into the audio editor**. Tick **Don't ask again** to skip this question next time.
3. Play. Notes go into the current pattern, quantized to the main snap. A take replaces the notes that start inside it.

**Record options** are in the OPTIONS menu, on the toolbar and in Settings ▸ Recording:

| Option | Shortcut | Effect |
|---|---|---|
| Count-in | Ctrl+P (except when the arranger is focused) | 1 or 2 bars of metronome before recording; the clock shows "REC IN n" |
| Start on input | Ctrl+I | Playback waits for the first note |
| Blend recorded notes | Ctrl+B (outside the piano roll and arranger) | A take keeps the notes it overlaps |
| Loop record | (none) | Playback loops, and each pass is a take |

Other recording tools:

- **TOOLS ▸ Cancel recording** undoes the takes of the current playback.
- **TOOLS ▸ Capture MIDI into pattern** writes the last 2 to 30 minutes of notes you played (the score log) into the
  pattern. **Clear log** empties the score log.

### 10.2 Recording audio

**Quick record** (the toolbar's microphone, or TOOLS ▸ Quick record):

1. Pick an input.
2. The song plays with Record armed and the count-in.
3. When you stop, the take becomes a 32-bit WAV in the data folder's `Recorded` folder, plus a new audio clip channel and
   clip where playback started.

If you choose **Into the audio editor**, the take goes into Tesla on the master track instead.

**Recording into mixer tracks:**

1. Arm tracks with their disk-record button, or with the track menu's **Arm selected track(s)**.
2. Arm Record and play the song. Every armed track records at its record source.
3. When you stop, each take becomes a WAV in `Recorded`. With the mixer menu's **Disk recording ▸ Auto-create audio
   clips** on, each take also becomes an audio clip.

**Input monitoring:** a track's **Monitor external input** setting (Off / When armed / On) decides when its input plays
through it.

**Retro audio capture** (TOOLS menu, on by default) keeps the last 1 to 10 minutes of what the audio device played.
**Open capture in audio editor** puts it into the master track's Tesla.

Loop recording and take lanes for audio are not available yet.

### 10.3 Tesla, the audio editor

Tesla sits in a mixer slot. It records what reaches its slot, holds one sample with markers, regions and a loop, and
edits it with undo. You can open it with:

- The toolbar's audio editor button.
- The mixer track menu's **Open audio editor**.
- **Open audio editor and record**, which sets Tesla up to record while the song plays, keeping the last 5 minutes.

**TOOLS ▸ Remove all audio editor instances** removes every Tesla after asking.

**Files:**

- Tesla loads WAV, AIFF and FLAC files.
- It saves WAV, including 24-bit, with markers, regions and loop.
- Drag the sample or selection out as a file, or use **Tools ▸ Sequencing**: **Send to arranger as audio clip**
  (Shift+C), **Send to selected channel** or **Send to new Lemon Slicer channel**.

**Tesla's menus** (shortcuts as shown in its menus):

| Menu | Entries |
|---|---|
| File | New, Load sample... (Ctrl+O), Save sample as... (Ctrl+S), Save sample as 24Bit .wav file..., About Tesla |
| Format | Edit properties... (F2), 16Bit int / 32Bit float, Mono / Stereo |
| Edit | Undo (Ctrl+Z), Redo (Ctrl+Y), Undo history, Undo mix (envelope), Cut (Ctrl+X), Copy (Ctrl+C), Paste insert (Ctrl+V), Paste replace (Alt+V), Paste mix (Shift+V), Paste mix (envelope) (Shift+Ctrl+V), Clear (Shift+Del), Delete (Del), Trim (Ctrl+Del), Delete part after loop (Alt+Del), Insert silence (Ins), Click-free (smooth) editing (C) |
| Tools: envelope and amp | Cancel all envelopes (Alt+E), Add points at selection (P), Mixdown amp envelopes (Ctrl+E), Amp... (Alt+A), Normalize (Ctrl+N), Fade in (Ctrl+F), Fade out (Alt+F), Reverse polarity, Declick in / out |
| Tools: time | Reverse (Alt+Left), Time stretch / pitch shift... (Alt+T), Drum(loop) stretch... (Ctrl+T), Scratch (envelope) |
| Tools: noise and spectral | Acquire noise threshold (Alt+N), Gate noise, Trim side noise, Trim all noise & slice up, Convolution reverb... (Ctrl+R), Blur... (Ctrl+B), Equalize... (Ctrl+Q), Acquire noise profile (Alt+U), Clean up (denoise)... (Ctrl+U), Vocal denoise / isolator..., Deverb... |
| Tools: other | Swap channels, channel conversions, Generate noise, region tools, Convert to score and dump to piano roll |
| Regions | Add region... (Alt+M), Add marker... (M), Set loop (Alt+L), Remove loop, Set first downbeat, Delete (Shift+Ctrl+Del), Auto rename all, Auto-slice, Detect beats, Detect pitch regions, Zero-cross check all regions |
| View | Spectrum (S), display settings, Regions, Loop, Time format |
| Snap | Snap to grid / regions / samples / zero-crossing (1 to 4) |
| Select | Deselect (Up), Select zoomed part (Ctrl+A), Select all, previous / next region (Left / Right) |
| Zoom | Zoom in / out (PgUp / PgDn), Zoom out full (Home), Zoom on selection (End), Zoom on left / right of selection (Ctrl+Left / Right) |

Edit ▸ Undo mix (spectral) is not available yet.

---

## 11. Time stretching

Sampler channels and audio clips can be stretched to fit the project tempo.

- **Stretch methods:** all twelve stretch methods are played. One open-source stretcher (Signalsmith Stretch) handles
  every method, so the sound differs from other DAWs' stretchers.
- **Following the tempo:** audio clips take their new length and redraw their waveform when the sampler's stretch
  settings, the channel pitch or the tempo change, including under tempo automation.
- **Restretch all** (the rack's menu, Ctrl+Alt+R with the rack focused) refits every stretched sample and audio clip at
  once.
- **Arranger stretch mode:** Shift+M with the arranger focused.
- **Switching audio clip channels:** **TOOLS ▸ Set all audio clip channels to ▸** and **Switch selected audio clip
  channels to ▸** change audio clip channels in bulk.
- **In Tesla:** **Time stretch / pitch shift...** (Alt+T) changes the selection's pitch by up to 24 semitones and its
  length from a quarter to four times, with formant preservation.

---

## 12. Exporting audio and MIDI

### 12.1 Exporting audio

1. Choose **FILE ▸ Export audio...** (Ctrl+R) or **Export MP3...** (Shift+Ctrl+R). The toolbar's export button works
   too.
2. In the Export form, set the following:
   - **Name** and **Folder.** Each format adds its own extension.
   - **Play:** Song (the arrangement) or Pattern (one pass of the current pattern).
   - **Range:** Whole, or the time Selection.
   - **Tail:** Cut, Ring out, or Wrap (wraps the tail to the start, for a seamless loop).
   - **Formats:** one or more of WAV (16-bit, 24-bit or 32-bit float), FLAC (16 or 24-bit, compression levels 0–8), MP3
     and OGG Vorbis (64–320 kbps).
   - **Quality:** the sample rate (the device's rate, or 44.1–192 kHz), Channels (stereo, or mono merged, left or
     right) and Resampling (2-point linear to 512-point sinc).
   - **Extras:**
     - Dither: triangular dither on 16 and 24-bit files.
     - HQ for plugins: hosted plugins render in their high-quality offline mode.
     - Split mixer tracks: also writes every mixer track alone, as WAV and FLAC, into a stems folder. Include the master
       adds the master's own file.
     - Insert effects and Master effects: whether those effects run.
     - Arranger markers, Loop points, Note markers and Tempo: written into WAV files.
     - Open the folder when done.
     - Gapless (MP3).
3. Click **Export** (Enter). Esc closes the form without exporting.

Lemon Studio remembers your choices.

To render individual mixer tracks, use the mixer track menu's **File ▸ Render to wave file...**. It writes each
selected track's output to its own file.

> **Limitation:** the export's Resampling choice doesn't reach Lemon Sampler channels, which keep their own
> interpolation.

### 12.2 Exporting MIDI

- **FILE ▸ Export MIDI...** (Shift+Ctrl+M) exports the current pattern, or the song's pattern clips, with a track per
  channel.
- The piano roll's Tools menu has **Export as MIDI file...** for one channel.
- **TOOLS ▸ Set up for MIDI export** turns every instrument channel into a Lemon MIDI Out channel (one MIDI channel each,
  in rack order). It asks first, and the change is one undo step.

### 12.3 Importing MIDI

1. Choose **FILE ▸ Import ▸ MIDI file...**, or drop a `.mid` file onto the window.
2. The **Import MIDI data** form opens. Pick the tracks, the MIDI channels and the instrument (Lemon Trio, Citrus or
   Lemon Sampler).
3. Choose whether to make a new project or add the file to the current one. Adding it is one undo step.

---

## 13. MIDI controllers and MIDI learn

### 13.1 MIDI settings

Open **OPTIONS ▸ MIDI settings** (F10). There you:

- Enable MIDI input and output devices.
- Set each output device's **Port**.
- Set the **Performance mode MIDI channel**.

The OPTIONS menu also has **Enable MIDI remote control**, **Enable MIDI output** and **Enable MIDI master sync**.

### 13.2 Linking a control (MIDI learn)

1. Right-click a knob, fader or plugin parameter.
2. Choose **MIDI learn...** (in the menu's Remote control section).
3. Move the knob or fader on your hardware. The Remote control settings box opens. Enter accepts the link; Esc drops it.
4. To remove a link, choose **Remove link** on the same menu.

More ways to link controls:

- **Link all parameters...** (plugin window menu, slot menu, Mixer options) links many parameters to consecutive
  controllers in one go.
- **Multi MIDI learn** (OPTIONS menu or toolbar, Ctrl+J) links several controls in a row.
- **VIEW ▸ Remote control** lists every controller link in the browser.

Links are kept in the program settings (`settings.txt`), not in the project. **Override global mapping...** is not
available yet.

### 13.3 MIDI output

Lemon MIDI Out channels send their notes to the MIDI output devices whose Port matches the channel's port, while
**Enable MIDI output** is on. Its knob pages and controllers are not available yet.

**Not available yet:** device scripts and typed mapping formulas.

---

## 14. Project files

### 14.1 Saving and opening

| Command | Shortcut |
|---|---|
| FILE ▸ New / New from template ▸ | (none) |
| FILE ▸ Open... | Ctrl+O |
| FILE ▸ Save | Ctrl+S |
| FILE ▸ Save as... | Shift+Ctrl+S |
| FILE ▸ Save new version | Ctrl+N |
| FILE ▸ Save to new project folder... | (none) |
| FILE ▸ Save as template... | (none) |
| FILE ▸ Clean up and save | (none); deletes unused samples, recordings and renders from the data folder after asking, then saves |
| FILE ▸ Export zipped project... | (none); the project and its samples in one `.zip` |
| FILE ▸ Restore backup... | (none); opens the project's newest backup |
| Recent projects | Alt+1 to Alt+0 open the ten most recent; More ▸ lists older ones, fifty in all |

Some details:

- **Native format:** Lemon Studio saves projects as `.lemon` files.
- **Opening `.flp` files:** Lemon Studio opens `.flp` projects saved by FL Studio 10 and later. Saving never overwrites
  the `.flp`: Save asks for a name and offers the same name with `.lemon`.
- **Unknown data is kept:** anything Lemon Studio doesn't understand yet is kept and written back unchanged. This
  support is still young, so always work on copies.
- **File association:** on Windows, **General settings ▸ System ▸ Associate project files** makes `.lemon` and `.flp`
  files open with Lemon Studio.

### 14.2 Data folders and backups

- **Data folder:** a project can have its own folder for backups, copied samples, recordings (`Recorded`) and renders
  (`Rendered`). Set it up in Settings ▸ Project. The first save of a new project offers to make one.
- **Autosave:** in **File settings ▸ Backup**, Autosave backs up a changed project at its interval. Backups go into the
  user folder's `Projects/Backup` (or the data folder's `Backup`), keeping the newest copies up to the Maximum (100 by
  default). Saving over a file also keeps a copy of the old one there.
- **Large-file warning:** a save larger than the **Project data size warning** (100 MB by default) tells you the file's
  size.

### 14.3 Undo

There is one undo history for the whole project.

- Ctrl+Z undoes.
- Ctrl+Alt+Z redoes (with **Alternate undo mode** on).
- Ctrl+Shift+Z always redoes.
- The EDIT menu names the step it will undo.
- **VIEW ▸ Undo history** shows the steps in the browser. Click a step to go back or forward to it.
- **General settings ▸ Undo history** sets the maximum number of undo levels and related options.

With Alternate undo mode off, Ctrl+Z toggles the last step (undo, then redo, then undo), and Ctrl+Alt+Z goes one more
step back. Step parameters are not covered by undo yet.

---

## 15. Settings and themes

### 15.1 Settings pages

Open the settings from the OPTIONS menu. The pages are:

| Page | Title | Opens with |
|---|---|---|
| MIDI | MIDI input / output devices | OPTIONS ▸ MIDI settings, F10 |
| Audio | Audio devices / mixer | OPTIONS ▸ Audio settings |
| Recording | Recording options | (tab) |
| General | System / miscellaneous | OPTIONS ▸ General settings |
| File | Files & folders | OPTIONS ▸ File settings |
| Plugins | Plugin manager | OPTIONS ▸ Manage plugins |
| Theme | Look and feel | OPTIONS ▸ Theme settings |
| Project | Current project | OPTIONS ▸ Project general settings |
| Info | Project credits | OPTIONS ▸ Project info, F11 |
| Debug | Debugging log | OPTIONS ▸ Debugging log |
| About | About | HELP ▸ About... |

**Audio page:** choose the output device, sample rate and buffer length here. Other options:

- On Windows: WASAPI shared or exclusive **Mode**, ASIO drivers and **Show ASIO panel**.
- On Linux: **Mode ▸ JACK**.
- **Auto close** closes the output while another program is in front.
- The **CPU** section has Multithreaded mixer processing, Multithreaded generator processing and Auto-bypass when silent.

**General page:**

- **Display:** High visibility, Toolbars scaling, Smoothing and Animation refresh rate.
- **Input ▸ Manage shortcuts.**
- **Hints and warnings ▸ Manage messages.**
- **Undo history.**
- **Advanced:** includes Threaded project sample loading and Auto keep long audio on disk.

**File page:** Backup, and Browser extra search folders.

About 14 controls are still greyed out, including Mix in buffer switch, Triple buffer, Priority, Safe overloads, Playback
tracking, Language, External tools and Troubleshooting. About 11 more are kept but have no effect yet, for example
Resampling quality and Align tick lengths. The program's settings are stored in `settings.txt` in the user folder.

### 15.2 Themes

- **Skins:** choose one in **Settings ▸ Theme**. **Lemon** is lemon on neutral greys with rounded-square controls.
  **Citrus** is the look formerly called Classic.
- **Built-in themes:** there are 25: Lemon and Citrus (the skins' own colours), plus Graphite, Frost, Persimmon, Mango,
  Spearmint, Moss, Cinnamon, Dragonfruit, Afterlight, Blueshift, Low Profile, Charcoal, Salamander, Velour, Lagoon,
  Cobalt, Orchid, Jade, Moss Light, Cinnamon Light, Raspberry, Kiwi and Apricot.
- **Your own themes:** put `.flstheme` files in the `themes` folder in the user folder. They are listed after the
  built-in themes.
- **Adjustments:** **Theme ▸ Adjustments** edits the applied theme live with five knobs and eight colour circles. While
  the Theme page is open, Ctrl+Z / Ctrl+Shift+Z / Ctrl+Y undo and redo theme changes.
- **Plugin editors:** Lemon Studio's own plugin editors always use the Lemon design.
- **Not available yet:** a theme's HTML background, the event editor grid and the Option colour.

---

## 16. Keyboard shortcuts

These shortcuts are confirmed in the source. Some keys mean different things depending on which window is in front.
On macOS, modifier mapping (Ctrl vs Command) is unverified.

### 16.1 Global

| Key | Action |
|---|---|
| Space | Play / stop |
| Ctrl+Space | Play / pause |
| L | Pattern / song switch |
| R | Record (typing keyboard off) |
| F2 | Rename and colour the pattern (in the mixer: rename the track) |
| F4 / Shift+F4 / Ctrl+F4 | Next empty pattern / new empty pattern / next empty pattern (unnamed) |
| F5 / F6 / F7 / F9 | Arranger / Instrument rack / Piano roll / Mixer |
| F8, Alt+F8, Shift+F8, Ctrl+F8 | Plugin palette, Browser, Plugin library, Clip palette (defaults; can be swapped) |
| F10 | MIDI settings |
| F11 | Project info |
| Ctrl+F11 | Hide the toolbars |
| F12 / Alt+F12 / Ctrl+F12 | Close all windows / plugin windows / unfocused windows |
| Shift+F12 | Align all channel editors |
| Ctrl+F5 | Tap tempo |
| Alt+F7 | Touch controller |
| Ctrl+O / Ctrl+S / Shift+Ctrl+S / Ctrl+N | Open / Save / Save as / Save new version |
| Ctrl+R / Shift+Ctrl+R | Export audio / Export MP3 |
| Shift+Ctrl+M | Export MIDI |
| Alt+1 … Alt+0 | Open a recent project |
| Ctrl+Z | Undo (see [14.3](#143-undo)) |
| Ctrl+Alt+Z | Redo with Alternate undo mode on; one more step back with it off |
| Ctrl+Shift+Z | Redo, in either mode |
| Ctrl+T | Computer keyboard as MIDI keyboard |
| Ctrl+M | Metronome |
| Ctrl+P | Count-in (in the arranger: performance mode) |
| Ctrl+I | Start on input |
| Ctrl+B | Blend recorded notes (in the piano roll and arranger: duplicate) |
| Ctrl+E | Step edit |
| Ctrl+J | Multi MIDI learn |
| Shift+Alt+G | Grouped clips move together |
| Ctrl+H | Stop sound |
| Shift+Ctrl+H | Realign windows |
| Ctrl+F | Browser find |
| Shift+Ctrl+P | Open the pattern in the project browser |
| Shift+Ctrl+Insert / Shift+Ctrl+Delete | Insert / delete a pattern |
| Shift+Ctrl+Up / Down | Move the pattern up / down |
| Numpad + / − | Typing keyboard octave (typing keyboard on) |
| Esc | Close menus and dialogs, clear the piano roll selection |

> PATTERNS ▸ Clone shows Shift+Ctrl+C, but that key has no handler in the main window (in the arranger it is Make
> unique).

### 16.2 Instrument rack (rack in front)

| Key | Action |
|---|---|
| Up / Down | Select the previous / next channel |
| Alt+Up / Alt+Down | Move the selected channels |
| 1 … 0 | Mute channels 1–10 (typing keyboard off) |
| Ctrl+1 … Ctrl+0 | Solo channels 1–10 |
| PgUp / PgDn | Change the display filter |
| Alt+C / Alt+G / Alt+Z / Alt+U | Clone / group / collapse selected / expand all |
| Alt+M | Show the channel's mixer track |
| Alt+Delete | Delete the selected channels |
| Ctrl+L | Route the selected channels to a free mixer track |
| Ctrl+X / Ctrl+C / Ctrl+V | Cut / copy / paste channels |
| Ctrl+K | Step parameters (graph editor) |
| Ctrl+Shift+Left / Right | Rotate the channel's steps |
| Ctrl+Alt+R | Restretch all |
| Alt+Shift+C / Ctrl+Alt+C | Gradient / random colour |
| Esc | Hide the rack |

### 16.3 Piano roll and arranger

See the tables in [5.1](#51-tools), [5.3](#53-the-tools-menu) and [6.2](#62-editing-keys-arranger-focused).

### 16.4 Mixer (mixer in front)

| Key | Action |
|---|---|
| Left / Right | Select the neighbouring track |
| Alt+Left / Alt+Right | Move the track |
| S / Alt+S | Solo / alt solo |
| F2 | Rename |
| Delete | Delete the track |
| Alt+W | Waveforms view |
| Alt+L | Select the channels routed to this track |
| Ctrl+L / Shift+Ctrl+L | Route the selected channels to this track / starting from it |
| Ctrl+Alt+L | Select the tracks routed to this one |
| Ctrl+A | Select all tracks |

---

## 17. Logs and troubleshooting

### 17.1 The user folder

Settings, themes, presets, backups, templates and logs live in the user folder. `OPENSTUDIO_USER_DIR` replaces it.

| System | Default user folder |
|---|---|
| Windows | `%APPDATA%\Lemon Studio` |
| Linux | `$XDG_CONFIG_HOME/Lemon Studio` (usually `~/.config/Lemon Studio`) |
| macOS | `~/Library/Application Support/Lemon Studio` |

### 17.2 Log files

Lemon Studio keeps several logs:

| Log | Location | What it holds |
|---|---|---|
| Application log | `<user folder>/logs/YYYY-MM-DD/YYYY-MM-DD_HH-MM-SS.log` | Every log line of one run. A new file starts each time you launch the program, and old files are never overwritten. `OPENSTUDIO_LOG_DIR` moves this folder. |
| Project log | `<folder containing the project file>/logs/YYYY-MM-DD/YYYY-MM-DD_HH-MM-SS.log` | The same lines, from the moment the project has a file on disk (opened, or saved for the first time). A new file starts each time the project is opened. Opening another project switches to that project's log. A project that has never been saved has no project log. |
| Panic log | `<user folder>/Logs/panic.log` | Crashes of Lemon Studio itself: time, version, thread, message and backtrace. Past 1 MB it restarts, keeping the previous one as `panic.log.1`. |
| Dropout log | `<user folder>/Logs/audio.log` | Playback problems: the audio settings each playback starts with, real-time priority, dropouts (at most one line a second) and plugins that made the audio thread wait. Past 256 KB it becomes `audio.log.1`. |

Log lines also go to the terminal (stderr) when you start Lemon Studio from one. Set `OPENSTUDIO_LOG=debug` for more
detail.

### 17.3 Diagnostics report

**HELP ▸ Diagnostics** writes a plain-text report to `Diagnostics.txt` in the user folder and opens it. The report covers
the build, your system, the audio settings, the plugins found and the open project.

### 17.4 Watching performance

- **CPU panel:** shows the audio load, voices, memory and dropouts. A dropout turns it red for a few seconds, and a "!"
  marks a plugin that arrived late.
- **Plugin performance monitor:** double-click the CPU panel to open it.

### 17.5 Common problems

- **A plugin is missing.**
  1. Check its folder against [8.3](#83-plugin-folders).
  2. Add the folder in Settings ▸ Plugins.
  3. Rescan (ADD ▸ Rescan plugins (fast)). The plugin manager's row hint says why a file failed.
- **A plugin channel is silent in a project from FL Studio.** FL plugins with no Lemon Studio version play silent, and
  their effect slots pass the sound through dry. The missing-files box doesn't list them yet.
- **Dropouts.** Raise the buffer length in Settings ▸ Audio. Check `Logs/audio.log` to see what happened.
- **Linux and macOS specifics.** See [Troubleshooting](https://getlemonstudio.app/wiki/reference/troubleshooting/) on
  the wiki, also reachable from HELP ▸ Troubleshooting.
- **Your built-in plugins show up twice in other DAWs.** They were renamed: delete bundles you copied yourself under the
  old names (Open Sampler, Open 3x Osc, Lemon 3x Osc and so on).

### 17.6 Reporting a bug

Use **HELP ▸ Report a bug** (it opens GitHub's bug report form). Attach:

1. **The project log** for the project you were working on: the newest file in `logs/YYYY-MM-DD/` next to the project
   file.
2. The **application log** of that run, from `<user folder>/logs/YYYY-MM-DD/`.
3. `Diagnostics.txt` from **HELP ▸ Diagnostics**.
4. `Logs/panic.log` if Lemon Studio crashed, and `Logs/audio.log` for sound problems.
5. Steps to reproduce the problem and, if possible, a small project (a copy) that shows it.

---

## 18. Scripting and assistants

The **control plane** lets scripts, tests and AI assistants read and change the open project. It uses JSON-RPC 2.0 over
a local socket: a Unix socket path on Linux and macOS, or a port number on any system (Windows takes ports only).

- Start the app with `--control <socket>` to control it beside the window. The hint bar shows where it listens.
- Start it with `--serve <socket> [project]` to run without a window.
- `--control-level` sets what clients may do:
  - `read`: look only.
  - `propose`: suggest changes. You **Apply** or **Discard** each proposal.
  - `edit` (the default): change the project directly.
- A port listens only on 127.0.0.1 and requires a token. You can set it with `--control-token`. Otherwise a new random
  token is generated each run and shown in the hint bar and terminal.
- Every change a client makes is one undo step, named after the client, so you can undo it like any other edit.
- Saving to a file asks you first while a window is open.

Example:

```bash
lemonstudio --control /tmp/lemon.sock song.lemon
lemonstudio --serve 7777 --control-level propose song.flp
```

The protocol and every command are documented in [docs/control-plane.md](control-plane.md).

Not available yet: live commands to the playing engine, and rendering through the control plane. VIEW ▸ Script output
and TOOLS ▸ Control builder are greyed out; Lemon Studio has no in-app scripting.

---

## 19. Known limitations

From `docs/status.md`, the README and the alpha checklist. These may change in newer builds.

- **Instrument rack:** no arpeggiator, echo delay, gate, time shift or tracking. Layer channel settings beyond volume and
  pan are missing. New or cloned channels are hidden under a group filter. Loop step channels / Loop all channels change
  nothing heard.
- **Piano roll:** no note colours, scale and chord helpers, or sound for release velocity (the Release lane changes
  nothing heard).
- **Arranger:** no per-clip pitch, pan and reverse, and no clip properties window. Consolidate and Export all playlist
  tracks leave plugins out.
- **Mixer:** only one mixer layout.
- **Plugin hosting:** no 32-bit plugins, Audio Units, Intel-only plugins on Apple silicon, or effect outputs past the
  main stereo pair. On macOS, a crash in a plugin's editor still closes Lemon Studio.
- **Built-in plugins:** some settings of the larger plugins are stored but not played.
- **Automation:** you can't edit a clip's targets, and ADD ▸ Automation clip makes a clip that can't be linked. You
  can't make controller links in the app.
- **Recording:** no loop recording or take lanes for audio.
- **MIDI:** no device scripts or typed mapping formulas.
- **Project files:** `.flp` support is not yet stable. Always work on copies.
- **Undo:** step parameters are not covered.
- **Settings:** some settings are greyed out or have no effect yet.
- **Menus:** some menus show keys that do something else. This affects the arranger's Select menu (Ctrl+Enter,
  Shift+C), the mixer track menu (Ctrl+E, Shift+E) and PATTERNS ▸ Clone (Shift+Ctrl+C).
- **Linux:** JACK has not been tried against a running JACK server.
- **Releases:** none have been published yet. Release builds will not be signed.

---

## 20. Glossary

| Term | Meaning |
|---|---|
| Arranger | The song timeline where clips are placed (called the playlist in some older documents). |
| Automation clip | A clip whose curve moves a control over time. |
| Channel | One instrument row in the instrument rack. |
| Clip | A placed copy of a pattern, an audio sample or an automation curve in the arranger. |
| Container | A slot or channel that holds several plugins played in series. |
| Control plane | The local JSON-RPC interface for scripts and assistants. |
| Data folder | A project's own folder for backups, samples, recordings and renders. |
| Instrument rack | The step-sequencer window listing all channels (called the channel rack in some older documents). |
| Mixer track | A strip in the mixer that channels and other tracks route into. |
| Pattern | A block of notes and steps for any number of channels. |
| PDC | Plugin delay compensation: making up the delay plugins add. |
| Performance mode | Live clip launching from the start of the arranger. |
| Sidechain | Feeding one track's audio into an effect on another track, for example for ducking. |
| Tesla | Lemon Studio's built-in audio editor and recorder. |
| Tick | The smallest unit of song time; the timebase sets how many make a quarter note. |
| User folder | Where Lemon Studio keeps settings, themes, presets and logs (see [17.1](#171-the-user-folder)). |

---

**Further reading:** the [Lemon Studio wiki](https://getlemonstudio.app/wiki/), and in this repository
[docs/status.md](status.md), [docs/development.md](development.md) and [docs/control-plane.md](control-plane.md).
