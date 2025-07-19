export default {
  extends: ["@commitlint/config-conventional"],
  rules: {
    "subject-case": [
      2,
      "never",
      [
        "lower-case",
        "upper-case",
        "camel-case",
        "kebab-case",
        "pascal-case",
        "snake-case",
        "start-case",
      ],
    ],
  },
};
