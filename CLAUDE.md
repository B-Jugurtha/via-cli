# Agent Guidelines & Project Roadmap: Rust Keyboard Modder (CLI)

## 🚨 Agent Constraints & Rules (Strict)

This is a **pure learning project** dedicated to mastering Rust. The developer's goal is comprehension, not just a working product.

- **ZERO CODE GENERATION:** Do not write, complete, or refactor any code blocks (Rust, TOML, Markdown configuration, etc.).
- **Role:** Act strictly as a mentor, architectural guide, and technical sounding board.
- **Assistance Style:** Provide high-level logic, pseudo-algorithms, architectural diagrams, troubleshooting steps, and conceptual hints. Never hand over the solution.

---

## 📌 Project Overview

The objective is to build a cross-platform command-line interface (CLI) application using **Rust** to dynamically configure keybindings, macros, and settings on a **Colorkey Reco A75** mechanical keyboard, driven entirely from the terminal.

---

## 🛠️ Step-by-Step Task Decomposition & Learning Roadmap

This roadmap breaks down the project into isolated milestones. **Crucial:** You must master the specified prerequisites before writing code for that step.

### ~~Phase 1: Hardware & Communication Discovery~~ ✅ COMPLETE

#### ~~Task 1.1: Identify the Device & Protocol~~ ✅
- VID `0x1480` / PID `0x6369`, VIA protocol over USB HID (usage page `0xFF60`, usage `0x0061`)

#### ~~Task 1.2: Establish a Rust-to-Keyboard Connection~~ ✅
- `hidapi` crate working; keyboard responds with VIA protocol v12

---

### Phase 2: Interactive CLI — Device Discovery & Info Display

Build the first real user-facing experience: an interactive two-level menu that lists connected keyboards by name and lets the user query device information.

#### Task 2.1: Modularise the Codebase

- **Description:** Break `src/main.rs` into a proper module tree (`device/`, `hid/`, `queries/`, `ui/`). Each module owns one responsibility. `main.rs` becomes a thin entry point.
- **🧠 Required Knowledge to Learn First:**
  - **Rust Modules:** `mod`, `pub`, `use`, and how `src/device/mod.rs` maps to `mod device` in `main.rs`.
  - **The `?` Operator:** Replace nested `match` chains with `?` so error propagation is clean before you add more callers.

#### Task 2.2: Device Discovery with Human-Readable Names

- **Description:** Enumerate HID devices filtered to the VIA interface only, and resolve each `(VID, PID)` pair to a recognisable keyboard name (e.g. `"Colorkey Reco A75"`). Fall back to the HID manufacturer/product strings for unknown devices.
- **🧠 Required Knowledge to Learn First:**
  - **Rust Structs & `impl` blocks:** Wrap raw `hidapi::DeviceInfo` in your own `struct` with a `friendly_name()` method.
  - **`Display` Trait:** `impl std::fmt::Display for YourStruct` so the struct prints its friendly name naturally.

#### Task 2.3: Two-Level Interactive Menu & Query Results

- **Description:** Screen 1 — select a keyboard by name. Screen 2 — pick a VIA query (`ProtocolVersion`, `FirmwareVersion`, `DeviceName`, …). Execute the query over HID and print a formatted result. No keybinding reads/writes yet.
- **🧠 Required Knowledge to Learn First:**
  - **`clap` (derive feature):** Entry-point argument parsing; subcommand enum pattern for future expansion.
  - **`inquire` or `dialoguer`:** Arrow-key selection lists. Read both READMEs and choose one.
  - **Rust Enums with Methods:** A `Query` enum where each variant knows its VIA command byte, how to build the 32-byte payload, and how to parse the response.
  - **Iterators:** `.map()` and `.collect()` to turn a `Vec<DeviceInfo>` into a `Vec<String>` of display names for the menu widget.

---

### Phase 3: Core Protocol & Data Serialization

Translating CLI commands into bytes the keyboard understands.

#### Task 3.1: Reverse Engineer or Implement the Config Protocol

- **Description:** Map out the exact byte packets required to change a keybinding or trigger a macro save on the Reco A75.
- **🧠 Required Knowledge to Learn First:**
  - **Byte Manipulation in Rust:** Bitwise operations (`&`, `|`, `^`), bit-shifting, and working with raw byte arrays (`[u8; N]`).
  - **Memory Layout & Endianness:** Understanding Big-Endian vs. Little-Endian when packing integers into byte streams.

#### Task 3.2: Data Serialization / Deserialization

- **Description:** Create Rust data structures representing layouts and macros, and serialize them into the protocol's byte format.
- **🧠 Required Knowledge to Learn First:**
  - **Rust Structs and Enums:** Creating expressive data types (e.g., a `Keycode` enum).
  - **The `Serde` Ecosystem:** How serialization works in Rust (even if doing custom byte packing, understanding the traits is essential).

---

### Phase 4: Keybinding & Macro Commands

Extending the CLI with commands that read and write keymaps.

#### Task 4.1: Argument Parsing & Command Dispatch

- **Description:** Add subcommands to the CLI (e.g., `keyboard_rs list-keys`, `keyboard_rs set-key <layer> <key> <action>`, `keyboard_rs save-macro`). Wire each subcommand's dispatch to the Phase 3 backend logic.
- **🧠 Required Knowledge to Learn First:**
  - **`clap` Subcommands in Depth:** Typed arguments, validation, and help generation beyond the basics introduced in Phase 2.
  - **Rust Enums as Command Variants:** Modeling each subcommand as an enum variant so a `match` block dispatches cleanly to the correct backend function.

#### Task 4.2: Interactive Keymap Editor

- **Description:** Extend the interactive menu from Phase 2 to let the user browse and edit keybindings without restarting the binary. Print formatted tables of the current layout.
- **🧠 Required Knowledge to Learn First:**
  - **Table-rendering crates:** e.g., `tabled` for presenting layout data clearly.
  - **Mutable state across menu iterations:** Keeping an in-memory keymap and flushing it to the keyboard on save.

---

### Phase 5: Async Architecture & Robustness

Ensuring the CLI doesn't block on hardware I/O.

#### Task 5.1: Background Threading for I/O

- **Description:** Move the USB/HID reading and writing to a background worker thread so the main thread can display a spinner or progress indicator while waiting, rather than hanging silently.
- **🧠 Required Knowledge to Learn First:**
  - **Concurrency in Rust:** Understanding the `Send` and `Sync` traits.
  - **Message Passing:** Using channels (`std::sync::mpsc` or `crossbeam`) to send configuration payloads from the main thread to the hardware thread.

---

## 🚦 How to Request Help from the Agent

When you hit a roadblock, ask for assistance using one of these formats:

1. _"I am stuck on Task X. Can you give me a pseudo-code logic flow for how the bytes should be shifted?"_
2. _"My compiler is throwing an ownership error about lifetimes when I pass a config struct into a command handler. Explain the concept I'm violating without rewriting my code."_
3. _"Give me an analogy of how `std::sync::mpsc` channels work so I can design my thread communication."_
