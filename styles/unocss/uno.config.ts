import { defineConfig, presetMini } from "unocss";

export default defineConfig({
  cli: {
    entry: {
      patterns: ["app/**/*.rs"],
      outFile: "public/style.css",
    },
  },
  presets: [presetMini()],
});
