# otdr-fixer

Got a bunch of .sor files with identifiers in the filename but not the actual files? Want to add comments, or correct typos, and don't want to drop tons of cash on expensive OTDR processing software? Need something you can use in a batch script?

Then `otdr-fixer` is for you!

Currently `otdr-fixer` can edit basic identifiers and other parameters in a SOR file's general parameters block, but because of its use of [otdrs](https://github.com/JamesHarrison/otdrs) as a backend, it can easily be extended for other purposes.

## Installation



## Usage

`otdr-fixer` is a command-line tool with one subcommand (`set_identifiers`) which is used to set identifiers. If identifiers are not explicitly modified, then they are left alone.

As an example, `otdr-fixer set_identifiers -a "A End" -b "B End" --comment "Hello from otdr-fixer" -i in.sor -o out.sor` will modify the origin and terminating locations and the comment from `in.sor` and write a new file to `out.sor` with the modified data.

```
USAGE:
    otdr-fixer set_identifiers [OPTIONS] --in_path <in_path> --out_path <out_path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cable_id <cable_id>                            Set the cable identifier
        --comment <comment>                              Set the free text comment
    -f, --fibre_id <fibre_id>                            Set the fibre identifier
    -i, --in_path <in_path>
        --operator <operator>                            Set the operator name for the trace
    -a, --originating_location <originating_location>
            Set the originating location of the trace, aka Location A

    -o, --out_path <out_path>
    -b, --terminating_location <terminating_location>
            Set the terminating location of the trace, aka Location B
```

## License

GPLv3 has been selected specifically to drive improved open source engagement with equipment manufacturers and developers of OTDR processing software in an industry that has struggled with open data exchange, proprietary (and vendor-locked) software, and poor maintenance of existing software.

otdr-fixer - a SOR file parsing tool
Copyright (C) 2021 James Harrison

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.