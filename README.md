
>
> **DISCLAIMER: This is work in progress and not yet production-ready!**
>

# merula

Merula is a plain-text, flat-file, human-editable database. The file
extension is '.mr'.

The location of the source directory is on
[github](https://github.com/nacl42/merula).

Merula files are suitable for semi-structured data, that is not yet
well defined, i.e. there is no definite structure (yet) and the kind
of information is subject to change.

The database consists of units of information called Memos with an
editor-friendly syntax. A single `memo` consists of a mandatory header
node and optional data nodes.

A very simple example looks like this:

```
@app merula
.url https://github.com/nacl42/merula
.tag, software, database, plain-text
.doc Merula is a plain-text, flat file database
.license MIT or Apache 2.0
```

This defines the memo with the title `merula` and which belongs to the
collection `app`. It has a `url`, a `doc` string and three different
`tag` nodes ("software", "database" and "plain-text").

More elaborate examples can be found in the `data` directory:

* Take a look at `data/periodic.mr` which gathers information from the
  periodic table of elements. Even if you are not into science, you
  will find that even though the number of elements is limited (118 at
  the time of writing), the data for each element can be quite
  extensive. While there are some data fields present for every
  element (such as the `name`), others may only be available for
  certain elements.
* The file `data/filter_periodic.mr` contains some pre-defined filters
  (using the `@mr:filter` directive). This is an example how the
  behaviour of merula can be extended from within the `.mr` file
  itself. The filter is only useful when using it alongside the
  original dataset, therefore it is included in `data/periodic.mr` by
  default (using the `@mr:include` directive).
* The file `data/apps.mr` lists other note-taking or simple database
  software. It shows off some of the features such as multi-line
  values (with or without explicit delimiter) and multi-value lines.

## Features and Limitations

Features are:
* editor-friendly, human-editable (easy to input)
* simple insertion of multiple nodes
* each node can have optional attributes
* simple yet effective query language (mql)
* emacs mode available (work in progress, not yet published on github)

Limitations are:
* only suitable for small database
* no nesting of fields

## Try it out!

Here are some sample queries. It is assumed that you are running this
from the merula directory.

```shell
# list all elements from the periodic table contained in the given file
$ cargo run -- list data/periodic.mr

# use mql expression to filter only alkaline metals
$ cargo run -- list data/periodic.mr --mql group=1

# same as above, but print memo contents as well
$ cargo run -- list data/periodic.mr --mql group=1 -v

# elements with a filled [Ar] shell
$ cargo run -- list data/periodic.mr --mql electrons~Ar

# elements with a density of more than 5
# as > implies a redirection, we should quote the mql expression
$ cargo run -- list data/periodic.mr --mql 'density>5'

# elements with atomic number between 80 and 90
$ cargo run -- list data/periodic.mr --mql 'number>=80,number<=90'

# elements using a pre-defined filter (see periodic.mr, @mr:filter)
$ cargo run -- list data/periodic.mr --filter ferromagnetic

# add another condition to the pre-defined filter
# again, remember to quote the condition as not to pipe the output...
$ cargo run -- list data/periodic.mr --filter ferromagnetic --mql 'amu>58'
```


## Roadmap

This is highly experimental software, which is used by the author and
probably no one else. Even the syntax itself is subject to change
(even though I assume that I will stick to `@header`, `.key` and
`+attribute`).

You have been warned.

Planned features are documented in the TODO file. If you want to read
the ones being worked on, used

```
$ cargo run -- list TODO.mr --filter active -v
```

The current implementation is meant to define the possible use cases
for the file format and for the command line utility. Speed is not a
priority, but maybe later on when the format and commands are fixed.

If you have any suggestions or feedback, you are welcome to do so via
the github page.


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
