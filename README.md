# merula

Merula is a plain-text, flat-file, human-editable database. The file
extension is '.mr'.

Merula files are suitable for data that is not yet well defined,
i.e. there is no definite structure (yet) and the kind of information
is subject to change.

The database consists of units of information called Memos with an
editor-friendly syntax. A single `memo` consists of a mandatory header
and optional data nodes.

A very simple example looks like this:

```
@app merula
.url https://github.com/nacl42/merula
.tag, software, database, plain-text
.doc Merula is a plain-text, flat file database
.note
This project is in its early stages.
Promising, but not yet ready for production.
Use at your own risk
.license gplv3
```

This defines the memo with the title `merula` and which belongs to the
collection `app`. It has a `url`, a `doc` string and three different
`tag` nodes ("software", "database" and "plain-text"). There is a
multiline node called `note` and also a `license` node.

## Features and Limitations

Features are:
* editor-friendly, human-editable (easy to input)
* simple insertion of multiple nodes
* each node can have optional attributes
* emacs mode available (work in progress, not yet published on github)

Limitations are:
* only suitable for small database
* no query language yet
* no nesting of fields

## Formal structure of a Memo

A single memo consists of a mandatory header node and optionally of
several data nodes.

The header node is of the form `@collection [title]`.

Data nodes can have different forms:
* single-line value: `.key value`
* multi-line value:
```
.key
line1
line2
line3
```

or

```
.key<<EOF
line1
line2
line3
.line4
EOF
```





