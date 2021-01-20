
# Goals

What are the goals for merula?

The following sections describe possible use cases. Based on these
scenarios, the goals can be defined.

Please note, that of course I am aware that all these use cases could
be solved with a spreadsheet, a simple database system, a yaml file, a
json file or with any other existing solution out there.

## Address Book

Consider the following excerpt from the personal address book file
`bilbo.mr` which belongs to Bilbo Baggins:

```
@contact Gandalf
.name Gandalf the Gray
.profession wizard
.species istari
.tag friend

@contact Thorin
.name Thorin Oakenshield
.profession treasure hunter
.species dwarf
.tag important

@contact Balin
.name Balin
.species dwarf
.profession treasure hunter

@contact Frodo
.name Frodo Baggins
.species hobbit
.profession bearer of the ring
.tag friend

@contact Pippin
.name Peregin Took
.species hobbit
.profession adventurer
.tag friend
```

List the names of all friends of Bilbo:

    $ merula bilbo.mr --select name --where 'tag~friend'

List all hobbits he knows:

    $ merula bilbo.mr --select name --where 'species=hobbit'


## Filter

Node filter could filter by data nodes or by data and header nodes.

In the latter case, we could distinguish between these:

```
@memo
.field
either-memo-or-field
```


```
[key][operator][value]

tag             has key 'tag'
tag~value       has key 'tag' which contains 'value'
~value          contains 'value' in any key
tag=value       has key 'tag' which equals to 'value'
tag>value       has key 'tag' with a value greater than 'value'
tag<value       has key 'tag' with a value smaller than 'value'
tag>=value      has key 'tag' with a value greater or equal to 'value'
tag<=value      has key 'tag' with a value smaller or equal to 'value'

.tag            same as above, but node is a data node
.tag~value

@tag            same as above, but node is a header node
@tag~value

+attr           has a node with an attribute 'attr'
+attr~value     has  anode with an attribute 'attr' which contains 'value'

.tag+attr       has a data node with a key 'tag' which has an attribute 'attr'
.+attr          has a data node with any key and an attribute 'attr'
```

The general form of a filter expression (fex) is

```
fex = { key_qualifier? ~ key? ~ ( op ~ value )? }
key_qualifier = { "@" | "." | "+" }
op = { "=" | "~" | ">=" | ">" | "<=" | "<" }
```
