/* eslint-disable no-console */
const path = require('node:path')
const swc = require('@swc/core')

swc
  .transform('useState(0)', {
    jsc: {
      parser: {
        syntax: 'typescript',
        tsx: true,
      },
      experimental: {
        plugins: [
          [
            path.join(
              __dirname,
              'target/wasm32-wasip1/release/swc_plugin_auto_import.wasm',
            ),
            {
              imports: ['react'],
              debug: true,
            },
          ],
        ],
      },
      minify: {
        compress: false,
        mangle: false,
      },
      // Try to disable transforms that might rename identifiers
      transform: {
        optimizer: {
          globals: {
            vars: {},
          },
        },
      },
    },
    minify: false,
    isModule: true,
  })
  .then((output) => {
    console.log('=== Transformed Code ===')
    console.log(output.code)
  })
  .catch((err) => {
    console.error('Error:', err)
  })
