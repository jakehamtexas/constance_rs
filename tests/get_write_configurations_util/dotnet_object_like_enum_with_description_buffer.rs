pub static DOTNET_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER1: &str = "namespace Constant\r\n{\r\n    public sealed class TestEnum\r\n    {\r\n        public readonly string First;\r\n        public readonly int Second;\r\n\r\n        private TestEnum(\r\n            string first,\r\n            int second\r\n        )\r\n        {\r\n            First = first;\r\n            Second = second;\r\n        }\r\n\r\n        /// <summary>\r\n        /// description1\r\n        /// </summary>\r\n        public static TestEnum Test1 = new TestEnum(\"first1\", 1);\r\n\r\n        /// <summary>\r\n        /// description2\r\n        /// </summary>\r\n        public static TestEnum Test2 = new TestEnum(\"first2\", 2);\r\n    }\r\n}\r\n";
pub static DOTNET_OBJECT_LIKE_ENUM_WITH_DESCRIPTION_BUFFER2: &str = "namespace Constant\r\n{\r\n    public sealed class TestEnum\r\n    {\r\n        public readonly string First;\r\n        public readonly int Second;\r\n\r\n        private TestEnum(\r\n            string first,\r\n            int second\r\n        )\r\n        {\r\n            First = first;\r\n            Second = second;\r\n        }\r\n\r\n        /// <summary>\r\n        /// description2\r\n        /// </summary>\r\n        public static TestEnum Test2 = new TestEnum(\"first2\", 2);\r\n\r\n        /// <summary>\r\n        /// description1\r\n        /// </summary>\r\n        public static TestEnum Test1 = new TestEnum(\"first1\", 1);\r\n    }\r\n}\r\n";