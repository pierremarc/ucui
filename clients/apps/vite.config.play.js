import { resolve } from "path";
import { defineConfig } from "vite";
import basicSsl from "@vitejs/plugin-basic-ssl";

const name = "play";

export default defineConfig({
  root: resolve(__dirname, name),
  publicDir: resolve(__dirname, "assets"),
  base: `/${name}/`,
  build: {
    outDir: resolve(__dirname, "dist", name),
  },
  plugins: [basicSsl()],
});
