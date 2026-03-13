---
title: Compiler Tokens
---

1. Lexxing

A “lex token” is one classified unit of source code that the lexer has
recognized and tagged with a type.

“lex” refers to lexical analysis (the first stage of a compiler/interpreter that
breaks text into tokens).

A “lex token” is simply one of those enum variants produced by that stage,
representing a smallest unit of meaning such as a keyword, identifier, operator,
or literal.

```text
use foo as bar
```

```rust
[
    LexTok::Use,
    LexTok::Identifier("foo".into()),
    LexTok::Alias,
    LexTok::Identifier("bar".into()),
]
```

2. Parsing

Parsing is the step where a program takes a stream of tokens (like Use, Fn,
identifiers, (, ), etc.) and checks whether they form a valid structure
according to the language’s grammar.

After lexing has turned raw text into tokens, parsing:

- Reads the tokens in order and matches them against grammar rules (like “a
  function definition is fn + name + params + body”).

- Builds a tree structure (often called a parse tree or syntax tree) that
  represents how the program is organized: expressions, statements, blocks,
  functions, etc.

- Reports syntax errors if the token sequence doesn’t fit the grammar (missing
  ), extra ;, wrong keyword order, and so on).

In the pipeline you’re looking at:

- Lexing: characters → LexTok sequence

- Parsing: LexTok sequence → syntax tree (AST) that later stages (like type
  checking or code generation) will use

3. Evaluating

“evaluating” is the step where you actually run or compute the meaning of an
expression or program.

- After lexing: you have tokens.

- After parsing: you have a syntax tree (often an AST).

- Evaluating that AST means:

  - Walking the tree,

  - Applying operators (+, *, ==, etc.),

  - Looking up variable values, calling functions, handling control flow (if,
    while, etc.),

  - And producing a result (like 42, "hello", or some side effect like printing
    or updating state).

```
+------------------+
|   Source code    |
|  (text, .ling)  |
+------------------+
          |
          |  1. Lexing (tokenization)
          v
+------------------+
|   Tokens stream  |
| [LexTok::Use,    |
|  LexTok::Fn,     |
|  Identifier, ...]|
+------------------+
          |
          |  2. Parsing (syntax analysis)
          v
+---------------------------+
|  AST (syntax tree)        |
|  e.g. FunctionDef(        |
|    name, params, body )   |
+---------------------------+
          |
          |  3. Evaluating / Executing
          v
+---------------------------+
|  Program behavior         |
|  - results / return vals  |
|  - printed output         |
|  - changed state, etc.    |
+---------------------------+
```

### Question: What is the difference between an interpreter and a compiler?

- Compiler: Translates the whole program into machine code (or bytecode) before
  you run it, producing a separate executable or binary file.

- Interpreter: Reads your source code and executes it directly, usually line by
  line or statement by statement, without producing a standalone executable.

- Compiled programs usually run faster because all translation work is done
  ahead of time and the result is optimized machine code.

- Interpreted programs usually run slower because translation happens as you
  execute the code.
