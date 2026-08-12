import path from "node:path";
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      input: {
        home: path.resolve(__dirname, "index.html"),
        download: path.resolve(__dirname, "download/index.html"),
        privacy: path.resolve(__dirname, "privacy/index.html"),
      },
    },
  },
});
