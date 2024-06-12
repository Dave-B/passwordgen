<!--
SPDX-FileCopyrightText: 2024 David Balch <david@balch.co.uk>

SPDX-License-Identifier: GPL-3.0-only
-->

# About wordlist.txt

The contents of wordlist.txt are included into PasswordGen and used to generate passwords.

This word list file was generated using [SCOWL](http://wordlist.aspell.net/)
(Spell Checker Oriented Word Lists), which is derived from many sources under a BSD compatible license.
The combined work is freely available under a [MIT-like license](https://raw.githubusercontent.com/kevina/wordlist/master/scowl/Copyright).


## (Re-)generating wordlist.txt

To update/customise the wordlist, use/modify the following steps:

1. Checkout and `make` code from https://github.com/en-wl/wordlist.git

    (The executable can be found in the "scowl" directory.)

2. Run, stripping accents, removing "'s" suffixes and upper/lower case duplicates:

    `./mk-list --accents strip british 10 | grep -v "'" | sort -u | uniq -i > wordlist_full.txt`
3. Remove words shorter than three letters:

    `awk '{ if (length($0) > 2) print }' wordlist_full.txt > wordlist.txt`

4. Copy wordlist.txt to the src/assets directory.

5. Re-build PasswordGen (e.g. `cargo build`) incorporate the new word list.
