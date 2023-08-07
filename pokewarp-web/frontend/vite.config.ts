import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    proxy: {
      '/api': 'http://localhost:3000',
      '/login': 'http://localhost:3000',
      '/signup': 'http://localhost:3000',
    }
  },
  plugins: [vue()],
})
