import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
  plugins: [wasmPack('./rust')],
  esbuild: { jsxInject: `import React from 'react'` },
});
