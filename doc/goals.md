
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

Sample filter expressions:

| expression             | node filter                                            |
| ---------------------- | ------------------------------------------------------ |
| name                   | has key 'name'                                         |
| name ~ value           | has key 'name' and value contains 'value'              |
| name = value           | has key 'name' and value is equal to 'value'           |
| age >= 18              | has key 'age' and value is greater or equal 18         |
| age < 18               | has key 'age' and value is smaller than 18             |
| birthday >> 2021-11-01 | has key 'birthday' which is later than 1st Nov 21      |
| .name                  | has key 'name' and node is a data node                 |
| @element               | has key 'element' and node is a header node            |
| name[0]                | first name                                             |
| name[-1]               | last name                                              |
| name[*]                | any name index                                         |
| name[0-1],[0:1],[0..1] | name 0 to 1                                            |
| name[..-1]             | all but the last name                                  |

Possible other filter expressions could include filtering for node
attributes via + qualifier.

The general form of a filter expression (fex) is

```
fex = { key_qualifier? ~ key? ~ ( op ~ value )? }
key_qualifier = { "@" | "." | "+" }
op = { "=" | "~" | ">=" | ">" | "<=" | "<" }
```

Unresolved: How can we join multiple filters ?

One memo, each condition must be satisfied at least for one node: ```name~ium,amu>50```

