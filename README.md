# constance_rs

A CLI tool for building constants and enums for Typescript, .NET Core, and more.

# Matrix

The following is a table of currently implemented and fully tested output types for the tool.

| Implementations            | Description                                                                            | Dotnet  | Typescript | Rust    |
| -------------------------- | -------------------------------------------------------------------------------------- | ------- | ---------- | ------- |
| Simple                     | An enum with string keys and integer values                                            | [MSSQL] | [MSSQL]    | [MSSQL] |
| Simple w/ Description      | -                                                                                      | [MSSQL] | [MSSQL]    | [MSSQL] |
| String                     | An enum with string keys and string values                                             | [MSSQL] | [MSSQL]    | [MSSQL] |
| String w/ Description      | -                                                                                      | [MSSQL] | [MSSQL]    | [MSSQL] |
| Object-like                | An enum with a string key and multiple values per key, of mixed string or integer type | [MSSQL] | [MSSQL]    | []      |
| Object-like w/ Description | -                                                                                      | [MSSQL] | [MSSQL]    | []      |
