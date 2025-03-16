import react from '@vitejs/plugin-react-swc'
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        topLevelAwait(), wasm(),
        react(),
    ],
});
