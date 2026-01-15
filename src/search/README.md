markdown

### 1. Language Overview

This DSL is an expression-oriented language used to filter rows in a dataset. Every query must evaluate to a **Boolean** (or Null, which is treated as False).

It enforces strict typing (e.g., you cannot compare a Number to a String), with the exception of mixing Integers and Floats.

### 2. Syntax Grammar (EBNF-like)

```ebnf
query        ::= logic_or

logic_or     ::= logic_and { ("||" | "|" | "OR") logic_and }
logic_and    ::= comparison { ("&&" | "&" | "AND") comparison }

comparison   ::= range { operator range }* (* Chaining allowed: 5 < x < 10 *)

operator     ::= "==" | "=" | "!=" | "<>" | "<" | "<=" | ">" | ">=" | "~=" | "in" | "not in"

range        ::= unary [ ".." unary ]
               (* Ranges are only valid within 'in' expressions, e.g., 'id in 1..100' *)

unary        ::= ("!" | "NOT") unary    (* Logical NOT *)
               | "-" unary              (* Numeric Negation *)
               | atom

atom         ::= literal
               | identifier             (* Column name *)
               | "@row"                 (* Row index variable *)
               | function_call
               | "(" query ")"
               | list
               | atom method_chain      (* e.g., name.len() *)

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
| **Integer** | `1`, `-50`           | 64-bit signed integers.                                      |
| **Float**   | `1.5`, `-0.01`       | 64-bit floating point numbers.                               |
| **String**  | `"Shiroko"`, `"123"` | Must be double-quoted.                                       |
| **Boolean** | `true`, `false`      | Result of comparisons.                                       |
| **Null**    | `null`               | Represents missing data. Propagates through most operations. |

**Implicit Casting Rules:**

- **Integer <-> Float**: Comparisons between integers and floats are allowed (e.g., `1 == 1.0` is true).
- **Others**: Strict. `1 == "1"` is false (or a type error in inequality checks). Use explicit casting functions.

---

### 4. Operators

#### Comparison Operators

| Operator             | Description | Usage                                            |
| -------------------- | ----------- | ------------------------------------------------ |
| `==`, `=`            | Equality    | `id = 5`                                         |
| `!=`, `<>`           | Inequality  | `name != "Nonomi"`                               |
| `<`, `<=`, `>`, `>=` | Ordering    | `attack > 1000`                                  |
| `~=`                 | Regex Match | `name ~= "^Shi.*"` (LHS and RHS must be Strings) |
| `in`                 | Inclusion   | `school in ["Abydos", "Gehenna"]`                |
| `not in`             | Exclusion   | `id not in [1, 2, 3]`                            |

**Range Syntax & Chaining:**

- **Ranges**: The `..` operator creates a range, valid only within `in` checks.
- `level in 1..10` checks if `1 <= level <= 10`.

- **Chaining**: Standard mathematical range notation is supported.
- `50 < level <= 80` is syntactic sugar for `(50 < level) && (level <= 80)`.

#### Logical Operators

| Operator         | Description | Usage             |
| ---------------- | ----------- | ----------------- |
| `&&`, `&`, `AND` | Logical AND | `x > 5 && x < 10` |
| `                |             | `,`               |
| `!`, `NOT`       | Logical NOT | `!is_locked`      |

---

### 5. Special Variables

- **`@row`**: Returns the **1-based** index of the current row being evaluated.
- Example: `@row <= 10` (Limits results to the first 10 rows).

---

### 6. Standard Library & Functions

Functions can be called normally `len(name)` or using method syntax `name.len()`.

#### Casting & Types

- **`int(value)`**: Casts String/Float to Integer.
- `int("5") == 5`

- **`float(value)`**: Casts String/Integer to Float.
- `float("5.5") == 5.5`

- **`str(value)`** / **`string(value)`**: Converts any value to its string representation.
- **`date(string)`**: Parses a date string (`YYYY-MM-DD`) into a Unix timestamp (Integer).
- `date(release_date) > date("2023-01-01")`

#### String Operations

- **`len(string)`**: Returns the length of the string or blob.
- **`lower(string)`**: Converts string to lowercase.
- `name.lower() == "shiroko"`

- **`upper(string)`**: Converts string to uppercase.
- **`contains(haystack, needle)`**: Returns true if `needle` is found in `haystack`.
- `name.contains("Swimsuit")`

#### Null Handling

- **`is_null(value)`**: Returns `true` if the value is null. (Safer than `== null`).

---

### 7. Examples

**1. Basic Filtering**
Find characters with "Shiroko" in their name or exact ID match.

```text
name.contains("Shiroko") || id == 10010
```

**2. Ranges and Sets**
Find items with an ID between 100 and 200 using `in` with a range, or specific IDs.

```text
id in 100..200 || id in [500, 501]
```

**3. Complex Logic & Regex**
Find characters from specific schools who match a regex pattern.

```text
school in ["Abydos", "Trinity"] && name ~= ".*(Terror|Swimsuit).*"
```

**4. Date Comparison**
Find events starting after a specific date.

```text
date(StartDate) > date("2024-01-01")
```

**5. Row Limiting**
Select only the first 50 rows where the level is maxed.

```text
Level == 90 && @row <= 50
```

**6. Case-Insensitive Search**
Normalize string to lower case before comparing.

```text
name.lower().contains("arona")
```
