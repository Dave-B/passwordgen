<!--
SPDX-FileCopyrightText: 2024 David Balch <david@balch.co.uk>

SPDX-License-Identifier: GPL-3.0-only
-->

# PasswordGen

CLI tool to create passwords in bulk (default 250 per run),
using multiple dictionary words and a configurable separator.

Uses word lists from [SCOWL](http://wordlist.aspell.net/) (MIT-like license).
See assets/wordlist.README.md for more details.

Originally created in Python to help a friend work with the sub-optimal
user management system they have to contend with.

Rewritten in Rust to start learning Rust, and to make a standalone executable for MS Windows.

## Building

Should be as simple as:

0. [Install Rust](https://www.rust-lang.org/tools/install).
1. Clone the repo.
2. Run `cargo build`.