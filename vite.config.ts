import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
    clearScreen: false,
    server: {
        port: 3000,
        strictPort: true
    },
    resolve: {
        alias: {
            '@': resolve(__dirname, './src')
        }
    },
    plugins: [vue()],
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
        target: ['es2021', 'chrome97', 'safari13'],
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG
    }
})
