# comparison with other note formats

## WikiData

WikiData is a free database that collects structured data to provide support for Wikipedia and related sites.

Each piece of information, called an Item, has a specific structure,
as described in the [WikiData
Introduction](https://www.wikidata.org/wiki/Wikidata:Introduction).

The comparison between WikiData Items and merula Memos shows a lot of
similarites:

| WikiData    | merula                           |
| ----------- | -------------------------------- |
| Item        | Memo                             |
| Identifier  | (implicit: collection and title) |
| Description | n/a                              |
| n/a         | Collection                       |
| Label       | Title                            |
| Alias       | n/a                              |
| Property    | Node                             |
| Value       | Value                            |
| Qualifiers  | Attributes                       |
| Rank        | (implicit: node order)           |

Each WikiData Item...
- ...describes one thing, in the same way that a Memo describes one
  topic.
- ... has a label and a description. The description exists to provide
  some help if there are two items with the same label. A Memo has a
  title and by default no additional description. However, you must
  provide the name of a collection and its purpose is also to provide
  a distinction between two memos with the same title but different
  collection.
- ... has an Identifier, which looks like (Q42). Since there is a
  central database, it can be assured that these identifiers are
  unique. Merula on the other hand (currently) does not provide such a
  unique identifier. It is assumed, that the combination of collection
  and title is unique, so this can be used to refer to another memo.
- ... provides several statements, which consists of
  property-value-pairs. This is very similar to the concept of nodes
  in merula, which consists of key-value pairs.
- ... can have additional qualifiers for each value. The qualifiers
  itself are distinct key-value pairs. Merula nodes can have
  attributes as well, also as key-value-pairs.
