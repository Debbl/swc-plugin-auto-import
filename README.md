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
            "presets": ["vue", "react"],
            "imports": {
              "@vueuse/core": ["useMouse", "useFetch"],
              "axios": [["default", "axios"]]
            },
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
          presets: ['react'],
          imports: {
            'lodash-es': ['debounce', 'throttle']
          }
        }
      ]
    ]
  }
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
                    presets: ['react']
                  }
                ]
              ]
            }
          }
        }
      }
    ]
  }
}
```

## 📖 Usage Examples

### Vue 3 Auto Import

**Configuration:**
```json
{
  "presets": ["vue"]
}
```

**Input:**
```js
const count = ref(0);
const doubled = computed(() => count.value * 2);

onMounted(() => {
  console.log('Component mounted');
});
```

**Output:**
```js
import { ref, computed, onMounted } from "vue";

const count = ref(0);
const doubled = computed(() => count.value * 2);

onMounted(() => {
  console.log('Component mounted');
});
```

### React Hooks Auto Import

**Configuration:**
```json
{
  "presets": ["react"]
}
```

**Input:**
```jsx
const [count, setCount] = useState(0);

useEffect(() => {
  console.log(count);
}, [count]);
```

**Output:**
```jsx
import { useState, useEffect } from "react";

const [count, setCount] = useState(0);

useEffect(() => {
  console.log(count);
}, [count]);
```

### Custom Library Import

**Configuration:**
```json
{
  "imports": {
    "@vueuse/core": ["useMouse", "useKeyboard"],
    "lodash-es": ["debounce", "throttle"],
    "axios": [["default", "axios"]]
  }
}
```

**Input:**
```js
const { x, y } = useMouse();
const debouncedFn = debounce(() => {}, 300);

axios.get('/api/data');
```

**Output:**
```js
import { useMouse } from "@vueuse/core";
import { debounce } from "lodash-es";
import { default as axios } from "axios";

const { x, y } = useMouse();
const debouncedFn = debounce(() => {}, 300);

axios.get('/api/data');
```

## ⚙️ Configuration Options

### `presets`

**Type:** `string[]`  
**Default:** `[]`

Preset configurations. Currently supported:
- `"vue"` - Vue 3 Composition API
- `"react"` - React Hooks
- `"vue-router"` - Vue Router Composition API
- `"react-router"` - React Router Hooks

### `imports`

**Type:** `Record<string, string[] | [string, string][]>`  
**Default:** `{}`

Custom import configuration.

**Format:**
```json
{
  "package-name": [
    "namedExport1",
    "namedExport2",
    ["exportName", "alias"]
  ]
}
```

**Example:**
```json
{
  "@vueuse/core": ["useMouse", "useFetch"],
  "axios": [["default", "axios"]],
  "date-fns": ["format", "parseISO"]
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
import { ref } from 'vue';
const count = ref(0);

// Output - unchanged
import { ref } from 'vue';
const count = ref(0);
```

### 2. No Import for Local Declarations

If an identifier is locally declared, the plugin won't add an import:

```js
// Input
function ref() {
  return "local ref";
}
const data = ref();

// Output - unchanged, no import added
function ref() {
  return "local ref";
}
const data = ref();
```

## 🔄 Comparison with unplugin-auto-import

| Feature | unplugin-auto-import | swc-auto-import |
|---------|---------------------|-----------------|
| Runtime | Vite/Webpack/Rollup | SWC Compiler |
| Performance | Fast | Very Fast |
| .d.ts Generation | ✅ | Planned |
| TypeScript Support | ✅ | ✅ |
| Custom Resolvers | ✅ | Planned |
| ESLint Integration | ✅ | Planned |

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
