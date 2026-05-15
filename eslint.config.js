import tseslint from "typescript-eslint";
import vueParser from "vue-eslint-parser";

const restrictInvokeImport = {
  "no-restricted-imports": [
    "error",
    {
      paths: [
        {
          name: "@tauri-apps/api/core",
          importNames: ["invoke"],
          message:
            "除 src/generated/bindings.ts 外，不允许直接导入 invoke，请改用生成的 bindings。",
        },
      ],
    },
  ],
};

export default [
  {
    files: ["src/**/*.{ts,tsx}"],
    ignores: ["src/generated/bindings.ts"],
    languageOptions: {
      parser: tseslint.parser,
      ecmaVersion: "latest",
      sourceType: "module",
    },
    rules: restrictInvokeImport,
  },
  {
    files: ["src/**/*.vue"],
    languageOptions: {
      parser: vueParser,
      ecmaVersion: "latest",
      sourceType: "module",
      parserOptions: {
        parser: tseslint.parser,
      },
    },
    rules: restrictInvokeImport,
  },
  {
    files: ["src/generated/bindings.ts"],
    languageOptions: {
      parser: tseslint.parser,
      ecmaVersion: "latest",
      sourceType: "module",
    },
    rules: {
      "no-restricted-imports": "off",
    },
  },
];