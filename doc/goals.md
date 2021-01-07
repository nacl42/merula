
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





