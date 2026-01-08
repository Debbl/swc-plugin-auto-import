// @ts-check
import { defineConfig } from '@debbl/eslint-config'

export default defineConfig({
  typescript: true,
  ignores: {
    files: ['tests/**', 'playground/**'],
  },
})
