# Keyboard RS — Learning Journal

---

## Phase 1: Hardware & Communication Discovery

**Status:** Complete
**Date:** 2026-06-13

---

### What I Built

A Rust CLI that:
1. Enumerates all HID devices on the system using the `hidapi` crate
2. Filters by the keyboard's VID (`0x1480`) and PID (`0x6369`), and narrows to the VIA interface by usage page (`0xFF60`) and usage (`0x0061`)
3. Opens a connection to that specific HID interface
4. Sends a 33-byte HID output report containing VIA command `0x01` (get protocol version)
5. Reads the 32-byte response and prints it

**Result:** The keyboard responded with `[1, 0, 12, ...]` — confirming VIA Protocol Version 12 is running on the firmware.

**Device identity discovered:**
- Retail name: Colorkey Reco A75
- True hardware name: HyphaRF KB369
- VID: `0x1480` / PID: `0x6369`
- Protocol: VIA over USB HID

---

### Concepts Learned

**USB HID Protocol**
- HID devices communicate through fixed-size reports: input (device → host), output (host → device), and feature reports
- The HID descriptor is sent by the keyboard on connect and declares the report size
- VIA uses 32-byte output and input reports

**HID Interfaces & Usage Pages**
- A single USB device can expose multiple HID interfaces
- Each interface is identified by a usage page and usage value
- The VIA configuration interface lives on usage page `0xFF60`, usage `0x0061`
- The OS keyboard driver owns the standard input interface — only the VIA interface is freely accessible

**Windows HID Quirk**
- On Windows, `hidapi`'s `write()` requires a prepended `0x00` report ID byte
- This makes the write buffer 33 bytes even though VIA reports are 32 bytes
- The keyboard ignores byte 0 — it is a Windows API requirement only

**Rust Patterns Used**
- `match` on `Result<T, E>` for error handling
- Nested match arms for chained fallible operations
- `[0u8; N]` for zero-initialised byte arrays
- `&array` to coerce a fixed-size array `[u8; N]` into a slice `&[u8]`
- `&mut array` for mutable slice references used in `read()`
- `eprintln!` for writing errors to stderr

**The `?` Operator (read, not yet applied)**
- A shortcut for propagating `Result` errors without nesting
- Requires the enclosing function to return `Result`
- Covered in The Rust Programming Language, Chapter 9.2

---

### What Comes Next — Phase 2

**Goal:** Understand and implement the VIA protocol well enough to read and write keybindings.

**Prerequisites to study before writing code:**
- Bitwise operations in Rust: `&`, `|`, `^`, `<<`, `>>`
- Big-Endian vs Little-Endian byte ordering
- Rust structs and enums for representing keycodes
- The `serde` ecosystem (even if doing manual byte packing)

**Key reference:** QMK firmware source — `quantum/via.h` lists all VIA command IDs and their byte layouts for Protocol v12.

**First task:** Map the byte packet required to read the current keymap from the keyboard (VIA command `0x04` — `id_get_keyboard_value` / keymap commands).
