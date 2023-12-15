pub static TYPESCRIPT_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER1: &str = "const TestEnum = {\r\n    /**\r\n     * description1\r\n     */\r\n    Test1: {\r\n        first: \"first1\",\r\n        second: 1,\r\n    },\r\n\r\n    /**\r\n     * description2\r\n     */\r\n    Test2: {\r\n        first: \"first2\",\r\n        second: 2,\r\n    },\r\n} as const;\r\n\r\nexport default TestEnum;\r\n";
pub static TYPESCRIPT_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER2: &str = "const TestEnum = {\r\n    /**\r\n     * description2\r\n     */\r\n    Test2: {\r\n        first: \"first2\",\r\n        second: 2,\r\n    },\r\n\r\n    /**\r\n     * description1\r\n     */\r\n    Test1: {\r\n        first: \"first1\",\r\n\r\n        second: 1,\r\n    },\r\n} as const;\r\n\r\nexport default TestEnum;\r\n";