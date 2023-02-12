import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()]
  // If using wsl2 outside wsl2 file system
  /*,
  server: {
    watch: {
      usePolling: true
    }
  }*/
});
