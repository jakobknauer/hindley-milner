A Rust implementation of [Algorithm J](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system#Algorithm_J) for type inference in the [Hindley-Milner type system](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system).

Furthermore, the repo includes:
- a parser for expressions and types,
- an equality check for types that respects renaming of bound variables,
- unit tests for the parser and the inference algorithm, and
- a REPL for inferring types of user-provided expressions (on empty context; for examples with non-empty context, see the unit tests in [src/algorithm_j.rs](src/algorithm_j.rs)).

Run the program to try out the REPL:
```
cargo run
...
>>> lambda x . x
⊢ λx . x : ∀ _1 . _1 → _1
>>> lambda f . lambda x . f x
⊢ λf . λx . f x : ∀ _3 _2 . (_2 → _3) → _2 → _3
>>> lambda f . lambda x . f (f x)
⊢ λf . λx . f (f x) : ∀ _4 . (_4 → _4) → _4 → _4
```

Caveats:
- The parser accepts the ASCII alternatives `lambda` for `λ`, `forall` for `∀`, and `to` for `→`.
- The parser only accepts alphanumeric identifiers starting with a letter.
- When parsing types, the parser interprets identifiers starting with a small letter as type variables, and identifiers starting with a capital letter as type constructors.
- As seen in the example above, fresh variables generated during inference are of the form `_1`, `_2`, etc., and thus cannot collide with parsed variables.
- The notation (identifiers in the code etc.) as well as the algorithm itself closely follow the description on Wikipedia.
