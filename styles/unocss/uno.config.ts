import { defineConfig, presetMini } from "unocss";

export default defineConfig({
  cli: {
    entry: {
      patterns: ["app/**/*.rs"],
      outFile: "public/uno.css",
    },
  },
  presets: [presetMini()],
});
