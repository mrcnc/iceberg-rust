<!--
  ~ Licensed to the Apache Software Foundation (ASF) under one
  ~ or more contributor license agreements.  See the NOTICE file
  ~ distributed with this work for additional information
  ~ regarding copyright ownership.  The ASF licenses this file
  ~ to you under the Apache License, Version 2.0 (the
  ~ "License"); you may not use this file except in compliance
  ~ with the License.  You may obtain a copy of the License at
  ~
  ~   http://www.apache.org/licenses/LICENSE-2.0
  ~
  ~ Unless required by applicable law or agreed to in writing,
  ~ software distributed under the License is distributed on an
  ~ "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
  ~ KIND, either express or implied.  See the License for the
  ~ specific language governing permissions and limitations
  ~ under the License.
-->


# Introduction

Iceberg Rust CLI (`iceberg-rust-cli`) is a small command line utility for interacting with Iceberg catalogs and
tables using functionality provided by iceberg-rust.  It does not support scanning tables and is primarily
intented to access table and catalog metadata from the command line.  To query tables from the command line you can use the playground.    

## Building

To build the binary, run the following from the root of the iceberg-rust repository

```
cargo build --release --bin iceberg-rust-cli
```