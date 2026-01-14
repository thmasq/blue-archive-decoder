### 1. Language Overview

This DSL is an expression-oriented language used to filter rows in a dataset. Every query must evaluate to a **Boolean** (or Null, which is treated as False). It enforces strict typing: you cannot compare a Number to a String without an explicit cast.

### 2. Syntax Grammar (EBNF-like)

```ebnf
query        ::= logic_or

logic_or     ::= logic_and { ("||" | "OR") logic_and }
logic_and    ::= comparison { ("&&" | "AND") comparison }

comparison   ::= unary { operator unary }
               | unary operator unary { operator unary }* (* Chaining: 5 < x < 10 *)

operator     ::= "==" | "!=" | "<" | "<=" | ">" | ">=" | "~=" | "in"

unary        ::= ("!" | "-" | "NOT") unary
               | atom

atom         ::= literal
               | identifier
               | function_call
               | "(" query ")"
               | list
               | atom method_chain

method_chain ::= "." identifier "(" [arguments] ")"

function_call::= identifier "(" [arguments] ")"
arguments    ::= query { "," query }

list         ::= "[" query { "," query } "]"

literal      ::= integer | float | string | boolean | "null"
string       ::= '"' { character | escape_sequence } '"'
escape_sequence ::= "\\" | '\"' | "\n" | "\r" | "\t"
```

---

### 3. Type System

The language operates on four primitive types. **Implicit casting is forbidden.**

| Type        | Examples             | Notes                                                        |
| ----------- | -------------------- | ------------------------------------------------------------ |
| **Integer** | `1`, `-50`, `0`      | Standard signed integers.                                    |
| **Float**   | `1.5`, `-0.01`       | Floating point numbers.                                      |
| **String**  | `"Shiroko"`, `"123"` | Must be double-quoted.                                       |
| **Boolean** | `true`, `false`      | Result of comparisons.                                       |
| **Null**    | `null`               | Represents missing data. Propagates through most operations. |

---

### 4. Operators

#### Comparison Operators

| Operator             | Description   | Usage                                            |
| -------------------- | ------------- | ------------------------------------------------ |
| `==`                 | Equality      | `id == 5`                                        |
| `!=`                 | Inequality    | `name != "Nonomi"`                               |
| `<`, `<=`, `>`, `>=` | Ordering      | `attack > 1000`                                  |
| `~=`                 | Regex Match   | `name ~= "^Shi.*"` (LHS and RHS must be Strings) |
| `in`                 | Set Inclusion | `school in ["Abydos", "Gehenna"]`                |

**Feature: Comparison Chaining**
Standard mathematical range notation is supported and syntactic sugar for `AND`.

- `50 < level <= 80` `(50 < level) && (level <= 80)`

#### Logical Operators

| Operator    | Description | Usage             |
| ----------- | ----------- | ----------------- |
| `&&`, `AND` | Logical AND | `x > 5 && x < 10` |
| `           |             | `,`OR`            |
| `!`, `NOT`  | Logical NOT | `!is_locked`      |

---

### 5. Standard Library & Functions

Functions can be called normally `len(name)` or using method syntax `name.len()`.

#### Casting & Types

- **`int(value)`**: Casts string/float to integer. Returns `null` if parsing fails.
- `int("5") == 5`

- **`str(value)`**: Converts any value to its string representation.
- **`date(string)`**: Parses a date string (e.g., "YYYY-MM-DD") into a timestamp (Integer).
- `date(release_date) > date("2023-01-01")`

#### String Operations

- **`len(string)`**: Returns the length of the string or blob.
- **`contains(haystack, needle)`**: Returns true if `needle` is found in `haystack`.
- `name.contains("Swimsuit")`

#### Null Handling

- **`is_null(value)`**: Returns `true` if the value is null. (Safer than `== null`).

---

### 6. Examples

**1. Basic Filtering**
Find characters with "Shiroko" in their name (case-insensitive depending on evaluator config) or exact ID match.

```text
name.contains("Shiroko") || id == 10010
```

**2. Numeric Ranges (Chaining)**
Find items with an ID between 100 and 200 (exclusive).

```text
100 < id < 200
```

**3. Complex Logic & Sets**
Find characters from specific schools who match a regex pattern.

```text
school in ["Abydos", "Trinity"] && name ~= ".*(Terror|Swimsuit).*"
```

**4. Handling Types (Strict Mode)**
Comparing a text column "Level" to a number requires casting.

```text
int(Level) >= 85
```

**5. Date Comparison**
Find events starting after a specific date.

```text
date(StartDate) > date("2024-01-01")
```
