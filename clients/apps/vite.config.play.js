import { resolve } from "path";
import { defineConfig } from "vite";
// import basicSsl from "@vitejs/plugin-basic-ssl";
import checker from "vite-plugin-checker";

const name = "play";

export default defineConfig(({ command }) => ({
  root: resolve(__dirname, name),
  publicDir: resolve(__dirname, "assets"),
  base: `/${name}/`,
  build: {
    outDir: resolve(__dirname, "dist", name),
  },
  plugins:
    command === "serve"
      ? [
          // basicSsl(),
          checker({
            typescript: true,
          }),
        ]
      : [],
}));
