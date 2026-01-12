# Import Types Reference

This document provides a comprehensive guide to all import types supported by swc-plugin-auto-import.

## Overview

The plugin supports **4 types of imports**:

1. **Named Import** - Import specific exports
2. **Named Import with Alias** - Import with a different local name
3. **Default Import** - Import the default export
4. **Namespace Import** - Import all exports as a namespace

## Configuration Formats

There are **3 ways to configure imports**:

1. **Preset String** - Use built-in presets
2. **Mapping Object** - Simplified syntax (recommended for most cases)
3. **Explicit Array** - Maximum flexibility

---

## 1. Named Import

Import specific named exports from a package.

### Mapping Format

```json
{
  "imports": [
    {
      "vue": ["ref", "computed", "reactive"]
    }
  ]
}
```

### Explicit Format

```json
{
  "imports": [
    [
      { "name": "ref", "from": "vue" },
      { "name": "computed", "from": "vue" },
      { "name": "reactive", "from": "vue" }
    ]
  ]
}
```

### Generated Code

```js
import { computed, reactive, ref } from 'vue'
```

---

## 2. Named Import with Alias

Import named exports with a different local name.

### Mapping Format

```json
{
  "imports": [
    {
      "@vueuse/core": [
        ["useFetch", "useMyFetch"],
        ["useMouse", "useMyMouse"]
      ]
    }
  ]
}
```

### Explicit Format

```json
{
  "imports": [
    [
      { "name": "useFetch", "as": "useMyFetch", "from": "@vueuse/core" },
      { "name": "useMouse", "as": "useMyMouse", "from": "@vueuse/core" }
    ]
  ]
}
```

### Generated Code

```js
import { useFetch as useMyFetch, useMouse as useMyMouse } from '@vueuse/core'
```

---

## 3. Default Import

Import the default export of a package.

### Mapping Format

```json
{
  "imports": [
    {
      "axios": [["default", "axios"]],
      "react": [["default", "React"]]
    }
  ]
}
```

### Explicit Format

```json
{
  "imports": [
    [
      { "name": "default", "as": "axios", "from": "axios" },
      { "name": "default", "as": "React", "from": "react" }
    ]
  ]
}
```

### Generated Code

```js
import axios from 'axios'
import React from 'react'
```

---

## 4. Namespace Import

Import all exports as a namespace object.

### Mapping Format

```json
{
  "imports": [
    {
      "lodash": [["*", "_"]],
      "framer-motion": [["*", "motion"]]
    }
  ]
}
```

### Explicit Format

```json
{
  "imports": [
    [
      { "name": "*", "as": "_", "from": "lodash" },
      { "name": "*", "as": "motion", "from": "framer-motion" }
    ]
  ]
}
```

### Generated Code

```js
import * as motion from 'framer-motion'
import * as _ from 'lodash'
```

---

## Complete Example

Mixing all import types together:

### Configuration

```json
{
  "imports": [
    "react",
    {
      "@vueuse/core": ["useMouse", ["useFetch", "useMyFetch"]],
      "axios": [["default", "axios"]],
      "lodash": [["*", "_"]]
    },
    [
      { "name": "ref", "from": "vue" },
      { "name": "useState", "as": "useSignal", "from": "react" },
      { "name": "*", "as": "motion", "from": "framer-motion" }
    ]
  ]
}
```

### Input Code

```ts
const count = ref(0)
const [value, setValue] = useSignal(10)
const { x, y } = useMouse()
const data = useMyFetch('/api/users')
const response = axios.get('/api/data')
const result = _.map([1, 2, 3], (n) => n * 2)
const animated = motion.div({ animate: { x: 100 } })
```

### Generated Output

```js
import { useFetch as useMyFetch, useMouse } from '@vueuse/core'
import axios from 'axios'
import * as motion from 'framer-motion'
import * as _ from 'lodash'
import { useState, useEffect } from 'react'
import { useState as useSignal } from 'react'
import { ref } from 'vue'

const count = ref(0)
const [value, setValue] = useSignal(10)
const { x, y } = useMouse()
const data = useMyFetch('/api/users')
const response = axios.get('/api/data')
const result = _.map([1, 2, 3], (n) => n * 2)
const animated = motion.div({ animate: { x: 100 } })
```

---

## Comparison: Mapping vs Explicit

### When to use Mapping Format

- ✅ Cleaner and more concise
- ✅ All imports are from the same package
- ✅ Recommended for most use cases

### When to use Explicit Format

- ✅ Maximum flexibility
- ✅ Imports from different packages in one array
- ✅ Better for programmatic generation

### Example Comparison

```json
// Mapping - cleaner for package-grouped imports
{
  "axios": [["default", "axios"]],
  "lodash": [["*", "_"]],
  "vue": ["ref", "computed"]
}

// Explicit - better for mixed sources
[
  { "name": "default", "as": "axios", "from": "axios" },
  { "name": "*", "as": "_", "from": "lodash" },
  { "name": "ref", "from": "vue" },
  { "name": "computed", "from": "vue" }
]
```

Both formats generate the same output!

---

## Quick Reference Table

| Type      | Mapping                | Explicit                                                | Output                               |
| --------- | ---------------------- | ------------------------------------------------------- | ------------------------------------ |
| Named     | `"ref"`                | `{ "name": "ref", "from": "vue" }`                      | `import { ref } from "vue"`          |
| Alias     | `["ref", "myRef"]`     | `{ "name": "ref", "as": "myRef", "from": "vue" }`       | `import { ref as myRef } from "vue"` |
| Default   | `["default", "axios"]` | `{ "name": "default", "as": "axios", "from": "axios" }` | `import axios from "axios"`          |
| Namespace | `["*", "_"]`           | `{ "name": "*", "as": "_", "from": "lodash" }`          | `import * as _ from "lodash"`        |
