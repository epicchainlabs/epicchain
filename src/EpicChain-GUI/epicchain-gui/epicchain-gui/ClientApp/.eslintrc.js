module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: ["eslint:recommended"],
  rules: {
    "no-console": ["warn", { allow: ["warn", "error"] }],
    "no-unused-vars": ["warn"],
    "no-empty": ["warn"]
  },
  parserOptions: {
    parser: "babel-eslint",
    ecmaVersion: 6,
  },
};
