
# Running the tests

$ cargo run -- test

```
# all elements
$ cargo run -- list data/periodic.mr

# only elements that contain a node with a key `ferromagnetic`
$ cargo run -- list data/periodic.mr --filter ferromagnetic
```
