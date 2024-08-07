
# Arboretum

An analysis framework for C++ Abstract Syntax Trees. 

## Components

### Reificator

A clang plugin which inspects clang itself to reify the AST data model and emit generated code which is used to interact with that data model.

## Data Model

Arboretum uses a [knowledge graph](https://en.wikipedia.org/wiki/Knowledge_graph) to represent the information it extracts from clang. This representation is a key feature that enables sophisticated techniques and analysis. 

### Enums

'enum ---- "class" ----> "clang::EnumDecl"

'enum ---- "enumerators" ----> 'list_node

'list_node ---- "class" ----> list

'list_node ---- 0 ----> /* first enumerator / <br/>
...<br/>
'list_node ---- N ----> / n+1'th enumerator */

### Reified Clang Nodes

/* Clang class / ---- methods ----> 'set_node

'set_node ---- "item" ----> / Clang method */

/* Clang method / ---- return type ----> / type */

### Const Method Outputs

'a ---- "class" ----> /* The clang class of 'a /

'a ---- / clang method fqn / ----> / various */

### Bool

The concepts of "true" and "false" are represented as named nodes in the graph. They have class "builtin".

'a ---- "class" ----> /* Clang class /

'a ---- / Method Fqn */ ----> "true"

### String

Strings are represented as named nodes.

'a ---- "class" ----> /* Clang class /

'a ---- / Method Fqn */ ----> "foo"

### Int64

Int64 are encoded directly as the lowest 64 bits of the Ulid id with the rest 0.

'a ---- "class" ----> /* Clang class /

'a ---- / Method Fqn */ ----> -42

### UInt64

Int64 are encoded directly as the lowest 64 bits of the Ulid id with the rest 0 except 65 which is set to 1.

'a ---- "class" ----> /* Clang class /

'a ---- / Method Fqn */ ----> 42

### Double

Doubles are encoded with the as the lowest 64 bits of the Ulid id with the rest 0 except 66 -> 1.

### List

A blank node with edges for each index element and one additional edge for "size".

### Set

A blank node with edges for each "item", and one additional edge for "size".