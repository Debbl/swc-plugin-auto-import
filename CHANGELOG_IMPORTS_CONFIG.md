# Configuration Update - Imports Field

## Summary

Updated the `imports` configuration field to support a more flexible TypeScript-like type system:

```typescript
imports?: Arrayable<ImportsMap | PresetName | InlinePreset>
```

## Changes Made

### 1. New Types in `config.rs`

#### `Arrayable<T>`
Supports both single value and array:
```rust
pub enum Arrayable<T> {
    Single(T),
    Array(Vec<T>),
}
```

#### `PresetImport`
Supports multiple formats for inline preset imports:
```rust
pub enum PresetImport {
    Simple(String),                    // "useState"
    Tuple(Vec<String>),                // ["useState", "useSignal"]
    Object { name, alias },            // { name: "useState", as: "useSignal" }
    Nested(Box<InlinePreset>),         // Nested inline preset
}
```

#### `InlinePreset`
Defines imports from a single module:
```rust
pub struct InlinePreset {
    pub from: String,
    pub type_only: Option<bool>,
    pub imports: Vec<PresetImport>,
}
```

#### Updated `ImportConfig`
Now supports all three main types:
```rust
pub enum ImportConfig {
    InlinePreset(InlinePreset),         // { from: "react", imports: [...] }
    Explicit(Vec<ExplicitImport>),      // [{ name: "ref", from: "vue" }, ...]
    ImportsMap(HashMap<...>),           // { "react": ["useState", ...] }
    PresetName(String),                 // "react"
}
```

#### Updated `PluginConfig`
```rust
pub struct PluginConfig {
    pub imports: Option<Arrayable<ImportConfig>>,  // Now optional and arrayable
    pub debug: bool,
}
```

### 2. Updated Processing Logic in `visitor.rs`

- Added `process_import_config()` - Process a single ImportConfig
- Added `process_inline_preset()` - Process InlinePreset format
- Added `process_preset_import()` - Process PresetImport with all its variants
- Updated `AutoImportVisitor::new()` - Handle Option<Arrayable<ImportConfig>>

### 3. New Test Cases

Created three new test fixtures:

1. **`inline-preset`** - Tests InlinePreset format with multiple PresetImport types
2. **`single-preset`** - Tests single preset string (non-array)
3. **`nested-inline-preset`** - Tests nested InlinePreset

All 13 tests pass successfully.

### 4. Updated Documentation

Updated README.md to document:
- New `Arrayable` type
- `InlinePreset` format with examples
- Nested preset support
- Type signature matching TypeScript definition

## Usage Examples

### Single Preset (Non-Array)
```json
{
  "imports": "react"
}
```

### InlinePreset Format
```json
{
  "imports": {
    "from": "react",
    "imports": [
      "useState",
      { "name": "useEffect", "as": "useReactEffect" },
      ["useMemo", "useMemoized"]
    ]
  }
}
```

### Nested InlinePreset
```json
{
  "imports": {
    "from": "react",
    "imports": [
      "useState",
      {
        "from": "react-dom",
        "imports": ["createPortal", "flushSync"]
      }
    ]
  }
}
```

### Mixed Array Format
```json
{
  "imports": [
    "react",
    { "react-dom": ["useFormStatus"] },
    {
      "from": "axios",
      "imports": [["default", "axios"]]
    }
  ]
}
```

## Backward Compatibility

All existing configurations remain compatible:
- Preset strings: `["react", "vue"]` ✅
- ImportsMap: `{ "react": ["useState"] }` ✅
- Explicit array: `[{ name: "ref", from: "vue" }]` ✅

## Benefits

1. **Flexibility** - Single value or array support
2. **Clarity** - InlinePreset makes configuration more readable
3. **Nesting** - Support for nested preset definitions
4. **Type Safety** - Matches TypeScript type system
5. **Backward Compatible** - Existing configs still work
