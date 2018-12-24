drpurge
===========

A utility to list files that are in both some directories and an
DaVinci Resolve project in order to delete then to save space.

How to build
------------

You need Rust.

Just do `cargo build` to build the tool.

Running
-------

You can run it with `cargo run -- -l path/to/drp -s
path/to/videos`.  It will print out all the files in `path/to/videos`
that are also found somewhere in the original medias for
`path/to/drp`.

Implementation details:
It compares the files based on name and byte size.


License
-------

  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.

See the LICENSE file in this repository.

Credit
------

Written by Hubert Figuiere <hub@figuiere.net>