# Hexalyzer

Hexalyzer is an app for viewing and modifying HEX files. Any binary encoded file
can technically be opened as well.

Hexalyzer project contains two main parts:
- A modern GUI application that can display and edit contents of HEX and BIN files.
- A standalone Intel HEX parsing library.


## Installation

Use [cargo packager](https://crates.io/crates/cargo-packager) to build an installer for your platform.

Go into hexalyzer directory `cd hexalyzer` and run:
1. `cargo build --release`
2. `cargo packager --release`

This will create `.exe` installer for Windows, and `.dmg` installer for
macOS. Linux was not tested but should work too.

When the tag is available, the installers will be attached.

### Notes for Windows

I had bunch of issues with generating Windows installer and the
final executable correctly. Before building, you need to run 
`cargo add --build winresource` which helps setup the icon on the
executable.

In addition, if you want to install the app in certain folders, e.g.,
`C:\Program Files\Hexalyzer`, you need to run the installer with
admin rights, the same goes for uninstalling.


## Usage

TBD


## History

v0.1.0 (2026-xx-xx) - Initial Release


## Future improvements

### UX and performance improvements to the app

1. Per-frame allocations in the hex grid

- Issue: app creates 16/32 `Button`s + 16/32 `Label`s per row; each byte formats strings (`format!("{:02X}")`); search, 
edits, etc. copy data around.

- Impact: hundreds of widgets and heap allocations per frame → high CPU pressure.

- Potential fix: render whole row as a single galley (`LayoutJob`) for hex and another for ASCII; detect hovered/selected 
cell via math on mouse position instead of individual widgets.

2. BTreeMap lookups in a tight render loop

- Issue: `ui_centralpanel.rs` repeatedly calls `ih.get_byte(addr)` per cell (`BTreeMap::get` has O(log n) time complexity).

- Impact: multiplied across all visible bytes each frame → wastes CPU.

- Fix: prefetch visible window once per frame into a small `Vec<Option<u8>>` and index into it.

3. Virtual scroll over (potentially huge) sparse ranges

- Issue: hex files can have address gaps, these gaps are displayed as empty rows.

- Impact: although it is used in some other hex viewer apps, it is not ideal for UX.

- Fix: compress gaps into a separator. The problem is that jump/search/etc offset calculations have to be adjusted accordingly.

4. Tabs are hacky to say the least...


### Architectural weaknesses

1. Data representated as `BTreeMap<usize, u8>`

- Memory-heavy and slow for contiguous data. Typical firmware is largely contiguous; a byte-per-node map scales poorly 
beyond ~100–300k bytes.

- Possible solution: represent data as contiguous segments `BTreeMap<usize, Vec<u8>>`, where key is the offset (aka start 
address of the contiguous segment) and value is the data vector.

2. UI tightly couples rendering and model details

- HexSession owns data, selection, editing, search, and paints per-byte widgets directly. Harder to unit test, refactor
and optimize.

- Possible solution: introduce a ViewModel layer: compute a lightweight "visible page" (bytes, ascii, selection masks)
separate from painting. The painter consumes this without hitting the data layer repeatedly.

3. Synchronous tasks on the main thread

- File load/save and potentially large searches run on the UI thread without the possibility to cancel.

- Possible solution: job system with background workers; add cancel and progress.

### Additional features

1. Support Copy, Undo, Redo, etc.
2. Support ELF format
3. Show the current address of the selected byte
4. Add timestamp (time since epoch) type in the data inspector
