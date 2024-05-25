# MusicMan : the comeback

## What is MusicMan ?
MusicMan is re-incarnation of my first ever somewhat solid project, musickid, a lightweight music player with vim-like bindings written in C that i never finished.
## Why rust ?
because rust is cool
## Build
- Clone the repo
- Crate alsa-sys requires dependencies :
```
 sudo apt-get install -y libasound2-dev portaudio19-dev build-essential libpulse-dev libdbus-1-dev 
```
## Progression
- [x] Display Hello world.
- [x] Play audio.
- [x] Parse mp3 for the data and print it to shell (title, artist, etc).
- [x] Display an empty window.
- [ ] Fill window with the parsed info and album art.
- [ ] Add ability to play multiple audio files (one after the other).
- [ ] Integrate the vim bindings.
