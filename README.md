
# Arboretum

An analysis framework for C++ Abstract Syntax Trees. 

## Components

### Reificator

In order to efficiently construct a complete data model for clang AST objects, the reificator is a clang plugin which inspects clang itself to reify the AST data model. The result is automatically generated code for the other components of the framework which is guaranteed to be consistent and complete.

