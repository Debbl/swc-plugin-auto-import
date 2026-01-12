# SWC Auto Import Plugin

An SWC plugin similar to [unplugin-auto-import](https://github.com/unplugin/unplugin-auto-import) that automatically imports APIs without manually writing import statements.

## 🚀 Features

- ✨ Auto import APIs from popular frameworks (Vue, React, Vue Router, etc.)
- 🎯 Support for custom import configuration
- 🔍 Smart detection: avoids duplicate imports and local declarations
- ⚡ Compile-time transformation with zero runtime overhead
- 🛠️ Native TypeScript support

## 📦 Installation

```bash
npm install swc-plugin-auto-import
# or
yarn add swc-plugin-auto-import
# or
pnpm add swc-plugin-auto-import
```

## 🔧 Configuration

### Configure in `.swcrc`

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        [
          "swc-plugin-auto-import",
          {
            "imports": [
              "vue",
              "react",
              {
                "@vueuse/core": ["useMouse", "useFetch"]
              },
              {
                "from": "axios",
                "imports": [["default", "axios"]]
              }
            ],
            "debug": false
          }
        ]
      ]
    }
  }
}
```

### Usage with Next.js

```js
// next.config.js
module.exports = {
  experimental: {
    swcPlugins: [
      [
        'swc-plugin-auto-import',
        {
          imports: [
            'react',
            {
              'lodash-es': ['debounce', 'throttle'],
            },
          ],
        },
      ],
    ],
  },
}
```

### Usage with Rspack

```js
// rspack.config.js
module.exports = {
  module: {
    rules: [
      {
        test: /\.(js|jsx|ts|tsx)$/,
        loader: 'builtin:swc-loader',
        options: {
          jsc: {
            experimental: {
              plugins: [
                [
                  'swc-plugin-auto-import',
                  {
                    imports: ['react'],
                  },
                ],
              ],
            },
          },
        },
      },
    ],
  },
}
```

## 📋 Import Types Summary

The plugin supports four types of imports:

| Import Type          | Mapping Format               | Explicit Format                                                      | Generated Code                                          |
| -------------------- | ---------------------------- | -------------------------------------------------------------------- | ------------------------------------------------------- |
| **Named**            | `"ref"`                      | `{ "name": "ref", "from": "vue" }`                                   | `import { ref } from 'vue'`                             |
| **Named with Alias** | `["useFetch", "useMyFetch"]` | `{ "name": "useFetch", "as": "useMyFetch", "from": "@vueuse/core" }` | `import { useFetch as useMyFetch } from '@vueuse/core'` |
| **Default**          | `["default", "axios"]`       | `{ "name": "default", "as": "axios", "from": "axios" }`              | `import axios from 'axios'`                             |
| **Namespace**        | `["*", "_"]`                 | `{ "name": "*", "as": "_", "from": "lodash" }`                       | `import * as _ from 'lodash'`                           |

## 📖 Usage Examples

### Vue 3 Auto Import

**Configuration:**

```json
{
  "imports": ["vue"]
}
```

**Input:**

```js
const count = ref(0)
const doubled = computed(() => count.value * 2)

onMounted(() => {
  console.log('Component mounted')
})
```

**Output:**

```js
import { ref, computed, onMounted } from 'vue'

const count = ref(0)
const doubled = computed(() => count.value * 2)

onMounted(() => {
  console.log('Component mounted')
})
```

### React Hooks Auto Import

**Configuration:**

```json
{
  "imports": ["react"]
}
```

**Input:**

```jsx
const [count, setCount] = useState(0)

useEffect(() => {
  console.log(count)
}, [count])
```

**Output:**

```jsx
import { useState, useEffect } from 'react'

const [count, setCount] = useState(0)

useEffect(() => {
  console.log(count)
}, [count])
```

### Custom Library Import

**Configuration (using package mapping):**

```json
{
  "imports": [
    {
      "@vueuse/core": ["useMouse", "useKeyboard"]
    },
    {
      "lodash-es": ["debounce", "throttle"]
    }
  ]
}
```

**Configuration (using explicit import array):**

```json
{
  "imports": [
    [
      { "name": "useMouse", "from": "@vueuse/core" },
      { "name": "useKeyboard", "from": "@vueuse/core" },
      { "name": "debounce", "from": "lodash-es" },
      { "name": "throttle", "from": "lodash-es" },
      { "name": "default", "as": "axios", "from": "axios" }
    ]
  ]
}
```

**Input:**

```js
const { x, y } = useMouse()
const debouncedFn = debounce(() => {}, 300)

axios.get('/api/data')
```

**Output:**

```js
import { useMouse } from '@vueuse/core'
import axios from 'axios'
import { debounce } from 'lodash-es'

const { x, y } = useMouse()
const debouncedFn = debounce(() => {}, 300)

axios.get('/api/data')
```

## ⚙️ Configuration Options

### `imports`

**Type:** `Array<string | object>`  
**Default:** `[]`

Import configuration. Supports three formats:

#### 1. Preset String (Built-in presets)

```json
{
  "imports": ["vue", "react", "vue-router", "react-router"]
}
```

Currently supported presets:

- `"vue"` - Vue 3 Composition API
- `"react"` - React Hooks
- `"react-dom"` - React DOM APIs
- `"vue-router"` - Vue Router Composition API
- `"react-router"` - React Router Hooks

#### 2. Package Mapping Object

A simplified syntax for defining imports. Each key is a package name, and the value is an array of imports.

```json
{
  "imports": [
    {
      "package-name": [
        "namedExport",
        ["exportName", "alias"],
        ["default", "defaultName"],
        ["*", "namespace"]
      ]
    }
  ]
}
```

**Example:**

```json
{
  "imports": [
    {
      "@vueuse/core": ["useMouse", ["useFetch", "useMyFetch"]]
    },
    {
      "axios": [["default", "axios"]]
    },
    {
      "lodash": [["*", "_"]]
    }
  ]
}
```

**Generated imports:**

```js
import { useFetch as useMyFetch, useMouse } from '@vueuse/core'
import axios from 'axios'
import * as _ from 'lodash'
```

**Format explanation:**

- **Named import**: `"useMouse"` → `import { useMouse } from '@vueuse/core'`
- **Named import with alias**: `["useFetch", "useMyFetch"]` → `import { useFetch as useMyFetch } from '@vueuse/core'`
- **Default import**: `["default", "axios"]` → `import axios from 'axios'`
- **Namespace import**: `["*", "_"]` → `import * as _ from 'lodash'`

> **Note**: The Mapping format is a simplified syntax. It's equivalent to the Explicit format but groups imports by package for cleaner configuration.
>
> ```json
> // Mapping format (concise)
> {
>   "axios": [["default", "axios"]],
>   "lodash": [["*", "_"]]
> }
>
> // Explicit format (verbose, same result)
> [
>   { "name": "default", "as": "axios", "from": "axios" },
>   { "name": "*", "as": "_", "from": "lodash" }
> ]
> ```

#### 3. Explicit Import Array

An array of import items where each item specifies the `name`, optional `as` (alias), and `from` (package) fields:

```json
{
  "imports": [
    [
      { "name": "ref", "from": "vue" },
      { "name": "useState", "as": "useSignal", "from": "react" },
      { "name": "default", "as": "_", "from": "lodash" },
      { "name": "*", "as": "lodash", "from": "lodash-es" }
    ]
  ]
}
```

**Generated imports:**

```js
import _ from 'lodash'
import * as lodash from 'lodash-es'
import { useState as useSignal } from 'react'
import { ref } from 'vue'
```

**Format explanation:**

- **Named import**: `{ "name": "ref", "from": "vue" }` → `import { ref } from 'vue'`
- **Named import with alias**: `{ "name": "useState", "as": "useSignal", "from": "react" }` → `import { useState as useSignal } from 'react'`
- **Default import**: `{ "name": "default", "as": "_", "from": "lodash" }` → `import _ from 'lodash'`
- **Namespace import**: `{ "name": "*", "as": "lodash", "from": "lodash-es" }` → `import * as lodash from 'lodash-es'`

#### Mixed Format

You can combine all three formats:

```json
{
  "imports": [
    "react",
    "react-dom",
    {
      "@vueuse/core": ["useMouse", "useFetch"]
    },
    [
      { "name": "default", "as": "axios", "from": "axios" },
      { "name": "computed", "from": "vue" }
    ]
  ]
}
```

### `debug`

**Type:** `boolean`  
**Default:** `false`

Enable debug mode (reserved field in current version).

## 📋 Built-in Presets

### Vue Preset

```
ref, computed, reactive, watch, watchEffect, onMounted, onUnmounted,
onBeforeMount, onBeforeUnmount, onUpdated, onBeforeUpdate, nextTick,
defineComponent, createApp, toRef, toRefs, unref, isRef
```

### React Preset

```
useState, useEffect, useContext, useReducer, useCallback, useMemo,
useRef, useLayoutEffect, useImperativeHandle
```

### Vue Router Preset

```
useRouter, useRoute
```

### React Router Preset

```
useNavigate, useLocation, useParams, useSearchParams
```

## 🎯 Smart Features

### 1. No Duplicate Imports

If an API is already imported, the plugin won't add it again:

```js
// Input
import { ref } from 'vue'
const count = ref(0)

// Output - unchanged
import { ref } from 'vue'
const count = ref(0)
```

### 2. No Import for Local Declarations

If an identifier is locally declared, the plugin won't add an import:

```js
// Input
function ref() {
  return 'local ref'
}
const data = ref()

// Output - unchanged, no import added
function ref() {
  return 'local ref'
}
const data = ref()
```

## 🔄 Comparison with unplugin-auto-import

| Feature            | unplugin-auto-import | swc-auto-import |
| ------------------ | -------------------- | --------------- |
| Runtime            | Vite/Webpack/Rollup  | SWC Compiler    |
| Performance        | Fast                 | Very Fast       |
| .d.ts Generation   | ✅                   | Planned         |
| TypeScript Support | ✅                   | ✅              |
| Custom Resolvers   | ✅                   | Planned         |
| ESLint Integration | ✅                   | Planned         |

## 🛠️ Development

### Build Plugin

```bash
# Build WASM plugin
cargo build-wasip1 --release

# Run tests
cargo test
```

### Project Structure

```
swc-plugin-auto-import/
├── src/
│   ├── lib.rs          # Plugin source
│   ├── config.rs       # Configuration
│   ├── presets.rs      # Presets
│   ├── collector.rs    # Identifier collector
│   └── visitor.rs      # AST visitor
├── Cargo.toml          # Rust configuration
├── package.json        # npm package config
└── README.md           # Documentation
```

## 📝 How It Works

1. **Scanning Phase**: Traverse AST to collect used, imported, and declared identifiers
2. **Matching Phase**: Find identifiers that need auto-import based on presets and custom config
3. **Filtering Phase**: Exclude already imported and locally declared identifiers
4. **Insertion Phase**: Insert generated import statements at the top of the module

## 🤝 Contributing

Issues and Pull Requests are welcome!

## 📄 License

ISC License

## 🔗 Links

- [SWC Official Documentation](https://swc.rs/)
- [unplugin-auto-import](https://github.com/unplugin/unplugin-auto-import)
- [SWC Plugin Development Guide](https://swc.rs/docs/plugin/ecmascript/getting-started)
