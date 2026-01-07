# Hexalyzer

Hexalyzer is an app for viewing and modifying HEX and BIN files.

Hexalyzer project contains two main parts:
- A modern GUI application that can display and edit contents of HEX and BIN files
- A standalone Intel HEX parsing library


## Installation

TBD


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
