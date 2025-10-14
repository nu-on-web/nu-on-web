import pluginSvelte from "eslint-plugin-svelte";
import parserSvelte from "svelte-eslint-parser";
import parserTs from "@typescript-eslint/parser";
import pluginTs from "@typescript-eslint/eslint-plugin";
import eslintConfigPrettier from "eslint-config-prettier/flat";

export default [
  // Ignore build artifacts
  {
    ignores: ["node_modules", "public/build", "dist", "src/wasm"],
  },
  // Lint Svelte components
  {
    files: ["**/*.svelte"],
    languageOptions: {
      parser: parserSvelte,
      parserOptions: {
        parser: parserTs,
        extraFileExtensions: [".svelte"],
        ecmaVersion: "latest",
        sourceType: "module",
      },
    },
    plugins: {
      svelte: pluginSvelte,
    },
    rules: {
      ...pluginSvelte.configs.recommended.rules,
    },
  },
  // Lint TS/JS files
  {
    files: ["**/*.ts", "**/*.js"],
    languageOptions: {
      parser: parserTs,
      parserOptions: {
        ecmaVersion: "latest",
        sourceType: "module",
      },
    },
    plugins: {
      "@typescript-eslint": pluginTs,
    },
    rules: {
      ...pluginTs.configs.recommended.rules,
    },
  },
  // Disable ESLint rules that conflict with Prettier
  eslintConfigPrettier,
];
