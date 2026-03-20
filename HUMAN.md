# DX Serializer Human Format Specification

**Version:** 2.0 (Leaf Inlining)  
**Status:** Production-Ready  
**Purpose:** Beautiful, readable format for developers to edit directly

## Overview

The Human Format is the source of truth for DX Serializer. It's designed to be:
- **Readable**: TOML/INI-like syntax with aligned equals signs
- **Editable**: Lives on real disk where you work (e.g., `dx`, `package.sr`)
- **Version-Controlled**: Commit these files to git
- **Auto-Converting**: Automatically generates `.llm` and `.machine` formats in `.dx/serializer/`

## Core Philosophy

1. **Human-First**: Optimized for reading and writing by developers
2. **Alignment**: Keys padded with spaces for visual alignment (typically column 28)
3. **Familiar Syntax**: TOML/INI-like with sections and key-value pairs
4. **Dot Notation**: Supports nested paths with dots (e.g., `forge.repository`)
5. **Source of Truth**: You edit these files; LLM and Machine formats are auto-generated

## File Locations

- **Human Format Files**: Live on **real disk** (e.g., `dx`, `package.sr`, `config.sr`)
- **LLM Format**: Auto-generated in `.dx/serializer/*.llm` (never edit manually)
- **Machine Format**: Auto-generated in `.dx/serializer/*.machine` (binary)
- **Git**: Only commit human format files; `.dx/` folder is gitignored

## Format Syntax

### 1. Root Key-Value Pairs (Scalars)

Simple key-value pairs at document root with aligned equals signs:

```dx
name                    = dx
version                 = 0.0.1
author                  = essensefromexistence
description             = Orchestrate dont just own your code
title                   = Enhanced Developing Experience
```

**Rules:**
- Spaces around `=` for readability
- Keys padded with spaces for alignment (typically at column 28)
- Use quotes `"..."` for strings with spaces
- Booleans: `true` or `false`
- Numbers: integers or floats
- Null: `null` or `none`

**Examples:**
```dx
port                    = 8080
active                  = true
timeout                 = 30.5
database                = null
title                   = "Enhanced Developing Experience"
```

### 2. Dot Notation (Leaf Inlining)

Use dots in keys for nested paths without section headers:

```dx
name                    = dx
version                 = 0.0.1
forge.repository        = https://dx.vercel.app/user/repo
style.path              = @/style
js.dependencies.react   = 19.0.1
js.dependencies.next    = 16.0.1
```

**Benefits:**
- No need for `[section]` headers for simple nested values
- Cleaner, more compact representation
- Dots are preserved as-is in the key name

### 3. Arrays

Two syntaxes for arrays:

#### Array Syntax 1: Colon with Dash Items

```dx
tags:
- rust
- performance
- serialization
```

#### Array Syntax 2: Count Notation

```dx
workspace.paths[2]:
- @/www
- @/backend

editors.items[7]:
- neovim
- zed
- vscode
- cursor
- antigravity
- replit
- firebase-studio
```

**Rules:**
- Key followed by `:` on its own line
- Optional `[n]` count before colon (e.g., `key[5]:`)
- Each item on a new line prefixed with `- `
- Items can be strings, numbers, or booleans
- Use quotes for multi-word items

**Examples:**
```dx
# Simple array
colors:
- red
- green
- blue

# Array with count
tools[3]:
- cli
- docs
- tests

# Array with multi-word items
editors:
- neovim
- "firebase studio"
- "visual studio code"
```

### 4. Sections

Group related key-value pairs under section headers:

```dx
[section_name]
key1                    = value1
key2                    = value2
```

**Rules:**
- Section headers in square brackets `[section]`
- Keys within sections are padded for alignment
- Nested sections use dot notation: `[section.subsection]`

**Example:**
```dx
[driven]
path                    = @/driven

[editors]
default                 = neovim
items:
- neovim
- zed
- vscode

[workspace]
paths:
- @/www
- @/backend

[forge]
repository              = https://github.com/user/repo
container               = none
pipeline                = none
tools:
- cli
- docs
- tests
```

### 5. Nested Sections

Use dot notation in section headers for nesting:

```dx
[js.dependencies]
react                   = 19.0.1
next                    = 16.0.1

[i18n.locales]
path                    = @/locales
default                 = en-US
```

### 6. Numbered Sections (Tables)

Create tables by using numbered section headers:

```dx
[dependencies:1]
name                    = dx-package-1
version                 = 0.0.1

[dependencies:2]
name                    = dx-package-2
version                 = 0.0.1

[dependencies:3]
name                    = dx-package-3
version                 = 0.0.2
```

**Conversion to LLM Format:**
```
dependencies[name version](
dx-package-1 0.0.1
dx-package-2 0.0.1
dx-package-3 0.0.2
)
```

**Rules:**
- Section name followed by `:` and a number (e.g., `[dependencies:1]`)
- All numbered sections with the same base name become table rows
- Schema is inferred from the keys in the first numbered section
- Subsequent sections must have the same keys

### 7. Boolean Values

Multiple representations for booleans:

```dx
# Standard
active                  = true
deleted                 = false

# Alternative (yes/no)
enabled                 = yes
disabled                = no

# Symbolic (+/-)
feature_a               = +
feature_b               = -
```

**Supported Values:**
- `true`, `yes`, `+` → boolean true
- `false`, `no`, `-` → boolean false

### 8. Null Values

```dx
database                = null
cache                   = none
```

**Supported Values:**
- `null`
- `none`

### 9. Comments

```dx
# This is a comment
name                    = dx  # Inline comments not supported

# Section header comments
# ═══════════════════════════════════════════════════════════
[configuration]
```

**Rules:**
- Lines starting with `#` are comments
- Inline comments (after values) are not officially supported
- Comment headers with `═` are decorative and ignored

## Complete Example

```dx
# DX Configuration File
# ═══════════════════════════════════════════════════════════

author                  = essensefromexistence
version                 = 0.0.1
name                    = dx
description             = Orchestrate dont just own your code
title                   = Enhanced Developing Experience

# Driven Configuration
[driven]
path                    = @/driven

# Editor Settings
[editors]
default                 = neovim
items[7]:
- neovim
- zed
- vscode
- cursor
- antigravity
- replit
- firebase-studio

# Workspace Configuration
[workspace]
paths[2]:
- @/www
- @/backend

# Forge Settings
[forge]
repository              = https://dx.vercel.app/essensefromexistence/dx
container               = none
pipeline                = none
tools[7]:
- cli
- docs
- examples
- packages
- scripts
- style
- tests

# Dependencies Table
[dependencies:1]
name                    = dx-package-1
version                 = 0.0.1

[dependencies:2]
name                    = dx-package-2
version                 = 0.0.1

# JavaScript Dependencies (Leaf Inlining)
js.dependencies.next    = 16.0.1
js.dependencies.react   = 19.0.1
```

## Type System

### Primitives

- **String**: Any text, use quotes for spaces
  ```dx
  name                  = dx
  title                 = "Enhanced Developing Experience"
  ```

- **Number**: Integer or float
  ```dx
  port                  = 8080
  timeout               = 30.5
  ```

- **Boolean**: `true`/`false`, `yes`/`no`, `+`/`-`
  ```dx
  active                = true
  enabled               = yes
  feature               = +
  ```

- **Null**: `null` or `none`
  ```dx
  database              = null
  cache                 = none
  ```

### Collections

- **Array**: List of values with `- ` prefix
  ```dx
  tags:
  - rust
  - fast
  ```

- **Object**: Section with key-value pairs
  ```dx
  [config]
  host                  = localhost
  port                  = 5432
  ```

- **Table**: Numbered sections with same base name
  ```dx
  [users:1]
  id                    = 1
  name                  = Alice
  
  [users:2]
  id                    = 2
  name                  = Bob
  ```

## Parsing Rules

### Key-Value Parsing

1. Find the `=` separator
2. Left side is the key (trimmed)
3. Right side is the value (trimmed)
4. Remove trailing comments starting with `  #`
5. Parse value based on type:
   - Quoted string: `"..."`
   - Boolean: `true`, `false`, `yes`, `no`, `+`, `-`
   - Null: `null`, `none`
   - Number: Parse as float
   - Array: `[...]` with comma-separated items
   - Default: Unquoted string

### Array Parsing

1. Detect array header: `key:` or `key[n]:`
2. Extract key name (remove `[n]` if present)
3. Read subsequent lines starting with `- `
4. Parse each item value
5. Stop at first non-item line

### Section Parsing

1. Detect section header: `[section_name]`
2. Check for numbered section: `[name:number]`
3. Parse key-value pairs until next section or EOF
4. For numbered sections, group by base name into tables

### Dot Notation Parsing

1. Keys with dots are preserved as-is
2. No automatic nesting (leaf inlining)
3. Example: `js.dependencies.react` stays as one key

## Conversion to LLM Format

### Scalars
```dx
# Human
name                    = dx
version                 = 0.0.1
```
```
# LLM
name=dx
version=0.0.1
```

### Arrays
```dx
# Human
tags:
- rust
- fast
```
```
# LLM
tags=[rust fast]
```

### Sections
```dx
# Human
[config]
host                    = localhost
port                    = 5432
```
```
# LLM
config(host=localhost port=5432)
```

### Tables
```dx
# Human
[users:1]
id                      = 1
name                    = Alice

[users:2]
id                      = 2
name                    = Bob
```
```
# LLM
users[id name](
1 Alice
2 Bob
)
```

## Best Practices

### DO ✅

1. **Use alignment** - Pad keys to column 28 for readability
2. **Use quotes** for multi-word strings
3. **Use dot notation** for simple nested values
4. **Use sections** for grouping related configuration
5. **Use numbered sections** for tabular data
6. **Add comments** to explain complex configuration
7. **Keep arrays vertical** - one item per line

### DON'T ❌

1. **Don't edit .llm or .machine files** - they're auto-generated
2. **Don't mix tabs and spaces** - use spaces only
3. **Don't use inline comments** after values (not officially supported)
4. **Don't manually calculate array counts** - parser handles it
5. **Don't nest sections deeply** - use dot notation instead

## Alignment Guidelines

### Standard Alignment (Column 28)

```dx
name                    = dx
version                 = 0.0.1
author                  = essensefromexistence
description             = Orchestrate dont just own your code
```

### Short Keys (Column 16)

```dx
name            = dx
version         = 0.0.1
port            = 8080
```

### Long Keys (No Padding)

```dx
very_long_configuration_key_name = value
another_long_key = another_value
```

**Rule of Thumb**: Choose alignment column based on your longest key, typically 16, 24, or 28.

## Security Considerations

### Input Size Limits

- Maximum input size: 100 MB (`MAX_INPUT_SIZE`)
- Maximum table rows: 1,000,000 (`MAX_TABLE_ROWS`)
- Parser validates size before processing

### Safe Parsing

- No code execution
- No external file inclusion
- No recursive includes
- Deterministic parsing

## Error Handling

### Common Parse Errors

1. **Invalid Section Header**
   ```dx
   [section  # Missing closing bracket
   ```

2. **Invalid Key-Value Pair**
   ```dx
   key without equals sign
   ```

3. **Invalid Table Format**
   ```dx
   [users:1]
   name = Alice
   
   [users:2]
   email = bob@example.com  # Different schema!
   ```

4. **Input Too Large**
   - File exceeds 100 MB limit

5. **Table Too Large**
   - Table exceeds 1,000,000 rows

## Migration from Other Formats

### From JSON

```json
{
  "name": "app",
  "version": "1.0",
  "tags": ["rust", "fast"],
  "config": {
    "host": "localhost",
    "port": 5432
  }
}
```

**To Human Format:**
```dx
name                    = app
version                 = 1.0

tags:
- rust
- fast

[config]
host                    = localhost
port                    = 5432
```

### From YAML

```yaml
name: app
version: 1.0
tags:
  - rust
  - fast
config:
  host: localhost
  port: 5432
```

**To Human Format:**
```dx
name                    = app
version                 = 1.0

tags:
- rust
- fast

[config]
host                    = localhost
port                    = 5432
```

### From TOML

```toml
name = "app"
version = "1.0"
tags = ["rust", "fast"]

[config]
host = "localhost"
port = 5432
```

**To Human Format:**
```dx
name                    = app
version                 = 1.0

tags:
- rust
- fast

[config]
host                    = localhost
port                    = 5432
```

## Advanced Features

### Prefix Inheritance (Legacy)

In older versions, `^` prefix indicated inheritance:

```dx
context.name            = app
^version                = 1.0
^title                  = My App
```

**Note**: This feature is legacy and may not be fully supported in v2.

### Table Parsing (Reserved)

The parser includes reserved methods for parsing formatted tables:

- Unicode box-drawn tables (`┌─┐│├┤└─┘`)
- ASCII tables (`+-|`)
- Markdown tables (`| --- |`)

**Note**: These features are reserved for future use and not currently active.

### Wrapped Row Continuation

For tables with long cell values, continuation rows are supported:

```dx
# First row
│ 1 │ Alice │ alice@example.com │
# Continuation row
│ ↓ │       │ (additional info) │
```

**Note**: This feature is reserved for future table parsing.

## Grammar (EBNF)

```ebnf
document        = (root_pair | section)* ;
root_pair       = key "=" value | key ":" array_items ;
section         = "[" section_name "]" section_content ;
section_name    = identifier | identifier ":" number ;
section_content = (pair | array_def)* ;
pair            = key "=" value ;
array_def       = key ":" array_items | key "[" number "]" ":" array_items ;
array_items     = ("- " value)+ ;
key             = identifier ("." identifier)* ;
value           = string | number | boolean | null | array_inline ;
string          = '"' [^"]* '"' | identifier ;
array_inline    = "[" (value ("," value)*)? "]" ;
boolean         = "true" | "false" | "yes" | "no" | "+" | "-" ;
null            = "null" | "none" ;
number          = [0-9]+ ("." [0-9]+)? ;
identifier      = [a-zA-Z_][a-zA-Z0-9_.-]* ;
```

## Implementation Notes

### Parser Architecture

1. **Line-by-Line Processing**: Parse input line by line
2. **State Tracking**: Track current section and entry order
3. **Type Inference**: Infer types from value syntax
4. **Entry Order Preservation**: Maintain insertion order for all entries

### Key Components

- `HumanParser`: Main parser struct
- `parse()`: Entry point for parsing
- `parse_key_value()`: Parse key-value pairs
- `parse_array_header()`: Detect array syntax
- `parse_section_header()`: Detect section headers
- `parse_config_section()`: Parse section contents
- `parse_config_value()`: Parse individual values

### Performance

- **Streaming**: Line-by-line processing for memory efficiency
- **No Backtracking**: Single-pass parsing
- **Lazy Evaluation**: Values parsed on demand
- **Size Limits**: Enforced to prevent DoS attacks

## Conclusion

The Human Format is designed to be the most readable and editable format for DX Serializer. It combines the familiarity of TOML/INI syntax with modern features like dot notation and array count annotations. By keeping human format files on disk and auto-generating LLM and Machine formats, DX Serializer provides the best of all worlds: readability for developers, token efficiency for AI, and performance for machines.

**Use Human Format for:**
- Configuration files developers edit
- Version-controlled settings
- Documentation examples
- Human-readable data storage

**Auto-Generated Formats:**
- LLM Format: Token-optimized for AI context windows (52-73% savings)
- Machine Format: Binary RKYV for maximum performance (zero-copy)

---

**Version:** 2.0 (Leaf Inlining)  
**Implementation:** Rust, production-ready  
**License:** MIT / Apache-2.0
