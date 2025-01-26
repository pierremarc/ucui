import { resolve } from "path";
import { defineConfig } from "vite";

const name = "monitor";

export default defineConfig({
  root: resolve(__dirname, name),
  publicDir: resolve(__dirname, "assets"),
  base: `/${name}/`,
  build: {
    outDir: resolve(__dirname, "dist", name),
  },
});
