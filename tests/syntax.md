# Markdown Code Blocks for Programming Languages

## Popular Programming Languages

### JavaScript
```javascript
function greet(name) {
    return `Hello, ${name}!`;
}
console.log(greet("World"));
```

### Python
```python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n-1) + fibonacci(n-2)

print(fibonacci(10))
```

### Java
```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
```

### C++
```cpp
#include <iostream>
using namespace std;

int main() {
    cout << "Hello, World!" << endl;
    return 0;
}
```

### C
```c
#include <stdio.h>

int main() {
    printf("Hello, World!\n");
    return 0;
}
```

### C#
```csharp
using System;

class Program {
    static void Main() {
        Console.WriteLine("Hello, World!");
    }
}
```

### TypeScript
```typescript
interface User {
    name: string;
    age: number;
}

const user: User = {
    name: "Alice",
    age: 30
};
```

### Go
```go
package main

import "fmt"

func main() {
    fmt.Println("Hello, World!")
}
```

### Rust
```rust
fn main() {
    println!("Hello, World!");
}

fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}
```

### Swift
```swift
import Foundation

func greet(name: String) -> String {
    return "Hello, \(name)!"
}

print(greet(name: "Swift"))
```

### Kotlin
```kotlin
fun main() {
    println("Hello, World!")
}

data class Person(val name: String, val age: Int)
```

### Ruby
```ruby
class Greeting
  def initialize(name)
    @name = name
  end
  
  def say_hello
    puts "Hello, #{@name}!"
  end
end

greeting = Greeting.new("Ruby")
greeting.say_hello
```

### PHP
```php
<?php
function calculateArea($radius) {
    return pi() * $radius * $radius;
}

echo "Area: " . calculateArea(5);
?>
```

### Perl
```perl
#!/usr/bin/perl
use strict;
use warnings;

my @numbers = (1, 2, 3, 4, 5);
my $sum = 0;
$sum += $_ for @numbers;
print "Sum: $sum\n";
```

### Scala
```scala
object HelloWorld {
  def main(args: Array[String]): Unit = {
    println("Hello, Scala!")
  }
}

case class Person(name: String, age: Int)
```

## Web Technologies

### HTML
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Hello World</title>
</head>
<body>
    <h1>Hello, World!</h1>
</body>
</html>
```

### CSS
```css
.container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
}

.greeting {
    font-size: 2rem;
    color: white;
    text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
}
```

### SCSS/Sass
```scss
$primary-color: #3498db;
$font-stack: 'Helvetica Neue', sans-serif;

.button {
    background-color: $primary-color;
    font-family: $font-stack;
    
    &:hover {
        background-color: darken($primary-color, 10%);
    }
}
```

### Less
```less
@primary-color: #428bca;
@margin: 10px;

.header {
    color: @primary-color;
    margin: @margin;
    
    .title {
        font-size: 2em;
        font-weight: bold;
    }
}
```

## Database Languages

### SQL
```sql
SELECT u.name, COUNT(o.id) as order_count
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
WHERE u.created_at > '2023-01-01'
GROUP BY u.id, u.name
HAVING COUNT(o.id) > 5
ORDER BY order_count DESC;
```

### MySQL
```mysql
CREATE TABLE products (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    price DECIMAL(10,2),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO products (name, price) VALUES ('Laptop', 999.99);
```

### PostgreSQL
```postgresql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    profile JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE INDEX idx_users_email_gin ON users USING gin(email gin_trgm_ops);
```

### MongoDB
```mongodb
db.users.aggregate([
    { $match: { age: { $gte: 18 } } },
    { $group: { _id: "$department", avgSalary: { $avg: "$salary" } } },
    { $sort: { avgSalary: -1 } }
]);
```

## Functional Languages

### Haskell
```haskell
quicksort :: Ord a => [a] -> [a]
quicksort [] = []
quicksort (x:xs) = 
    let smaller = quicksort [a | a <- xs, a <= x]
        bigger = quicksort [a | a <- xs, a > x]
    in smaller ++ [x] ++ bigger
```

### Clojure
```clojure
(defn factorial [n]
  (if (<= n 1)
    1
    (* n (factorial (dec n)))))

(map factorial (range 1 6))
```

### Erlang
```erlang
-module(hello).
-export([start/0]).

start() ->
    spawn(fun() -> loop() end).

loop() ->
    receive
        {hello, Name} ->
            io:format("Hello ~s!~n", [Name]),
            loop();
        stop ->
            ok
    end.
```

### Elixir
```elixir
defmodule Math do
  def sum_list([]), do: 0
  def sum_list([head | tail]), do: head + sum_list(tail)
end

IO.puts Math.sum_list([1, 2, 3, 4, 5])
```

### F#
```fsharp
let rec fibonacci n =
    match n with
    | 0 | 1 -> n
    | _ -> fibonacci (n-1) + fibonacci (n-2)

[1..10] |> List.map fibonacci |> printfn "%A"
```

### OCaml
```ocaml
let rec fact n =
  if n <= 1 then 1
  else n * fact (n - 1);;

let numbers = [1; 2; 3; 4; 5];;
let squares = List.map (fun x -> x * x) numbers;;
```

## System Programming

### Assembly (x86-64)
```assembly
.section .data
    hello: .ascii "Hello, World!\n"
    hello_len = . - hello

.section .text
    .global _start

_start:
    mov $1, %rax        # sys_write
    mov $1, %rdi        # stdout
    mov $hello, %rsi    # message
    mov $hello_len, %rdx # length
    syscall
    
    mov $60, %rax       # sys_exit
    mov $0, %rdi        # status
    syscall
```

### Zig
```zig
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    try stdout.print("Hello, Zig!\n");
}

fn fibonacci(n: u32) u32 {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}
```

### D
```d
import std.stdio;
import std.algorithm;
import std.range;

void main() {
    auto numbers = iota(1, 11);
    auto squares = numbers.map!(x => x * x);
    writeln("Squares: ", squares);
}
```

### Nim
```nim
proc fibonacci(n: int): int =
  if n <= 2:
    result = 1
  else:
    result = fibonacci(n - 1) + fibonacci(n - 2)

for i in 1..10:
  echo "fib(", i, ") = ", fibonacci(i)
```

## Scripting Languages

### Bash
```bash
#!/bin/bash

function backup_files() {
    local source=$1
    local dest=$2
    
    if [ ! -d "$dest" ]; then
        mkdir -p "$dest"
    fi
    
    cp -r "$source"/* "$dest"/
    echo "Backup completed: $source -> $dest"
}

backup_files "/home/user/documents" "/backup/documents"
```

### PowerShell
```powershell
function Get-SystemInfo {
    param(
        [string[]]$ComputerName = $env:COMPUTERNAME
    )
    
    foreach ($Computer in $ComputerName) {
        Get-WmiObject -Class Win32_ComputerSystem -ComputerName $Computer |
        Select-Object Name, Manufacturer, Model, TotalPhysicalMemory
    }
}

Get-SystemInfo
```

### Lua
```lua
function fibonacci(n)
    if n <= 1 then
        return n
    else
        return fibonacci(n-1) + fibonacci(n-2)
    end
end

for i = 1, 10 do
    print("fib(" .. i .. ") = " .. fibonacci(i))
end
```

### R
```r
# Load data
data(mtcars)

# Create a linear model
model <- lm(mpg ~ wt + hp, data = mtcars)

# Plot the results
plot(mtcars$wt, mtcars$mpg, 
     xlab = "Weight", ylab = "Miles per Gallon",
     main = "Car Weight vs Fuel Efficiency")
abline(model, col = "red")
```

## Mobile Development

### Dart (Flutter)
```dart
import 'package:flutter/material.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: Text('Hello Flutter')),
        body: Center(child: Text('Hello, World!')),
      ),
    );
  }
}
```

### Objective-C
```objc
#import <Foundation/Foundation.h>

@interface Calculator : NSObject
- (int)add:(int)a to:(int)b;
@end

@implementation Calculator
- (int)add:(int)a to:(int)b {
    return a + b;
}
@end

int main() {
    Calculator *calc = [[Calculator alloc] init];
    NSLog(@"Result: %d", [calc add:5 to:3]);
    return 0;
}
```

## Configuration & Markup

### YAML
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: app-config
  namespace: production
data:
  database_url: "postgresql://user:pass@localhost:5432/mydb"
  redis_url: "redis://localhost:6379"
  log_level: "info"
  features:
    - authentication
    - logging
    - monitoring
```

### JSON
```json
{
  "name": "my-project",
  "version": "1.0.0",
  "description": "A sample project",
  "main": "index.js",
  "scripts": {
    "start": "node index.js",
    "test": "jest",
    "build": "webpack --mode production"
  },
  "dependencies": {
    "express": "^4.18.0",
    "lodash": "^4.17.21"
  },
  "devDependencies": {
    "jest": "^28.0.0",
    "webpack": "^5.70.0"
  }
}
```

### XML
```xml
<?xml version="1.0" encoding="UTF-8"?>
<bookstore>
    <book id="1" category="fiction">
        <title lang="en">The Great Gatsby</title>
        <author>F. Scott Fitzgerald</author>
        <price currency="USD">12.99</price>
        <publication>
            <year>1925</year>
            <publisher>Scribner</publisher>
        </publication>
    </book>
</bookstore>
```

### TOML
```toml
[package]
name = "my-rust-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[profile.release]
opt-level = 3
lto = true
```

### INI
```ini
[database]
host = localhost
port = 5432
username = admin
password = secret123
database = myapp

[logging]
level = INFO
file = /var/log/myapp.log
max_size = 10MB
```

## Documentation & Markup

### Markdown
```markdown
# My Project

## Features

- **Fast**: Built with performance in mind
- **Secure**: Industry-standard encryption
- **Scalable**: Handles millions of requests

### Installation

```bash
npm install my-project
```

### Usage

```javascript
const myProject = require('my-project');
myProject.init();
```

## License

MIT License - see [LICENSE.md](LICENSE.md)
```

### LaTeX
```latex
\documentclass{article}
\usepackage{amsmath}
\usepackage{graphicx}

\title{Mathematical Analysis}
\author{John Doe}
\date{\today}

\begin{document}
\maketitle

\section{Introduction}
The quadratic formula is given by:
\begin{equation}
x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
\end{equation}

\end{document}
```

### reStructuredText
```rst
My Documentation
================

This is a sample document written in reStructuredText.

Features
--------

* **Bold text** and *italic text*
* Code blocks with syntax highlighting
* Mathematical expressions

.. code-block:: python

    def hello_world():
        print("Hello, World!")

.. note::
   This is an important note.
```

## Specialized Languages

### MATLAB
```matlab
function result = fibonacci_matrix(n)
    % Calculate fibonacci using matrix multiplication
    if n <= 1
        result = n;
        return;
    end
    
    F = [1 1; 1 0];
    result_matrix = F^(n-1);
    result = result_matrix(1,1);
end

% Plot fibonacci sequence
n = 1:20;
fib_values = arrayfun(@fibonacci_matrix, n);
plot(n, fib_values, 'bo-');
title('Fibonacci Sequence');
```

### Mathematica
```mathematica
(* Define a function to calculate pi using series *)
piApproximation[n_] := 4 * Sum[(-1)^k/(2*k + 1), {k, 0, n}]

(* Plot the convergence *)
Plot[piApproximation[n], {n, 1, 100}, 
 PlotLabel -> "Pi Approximation Convergence"]

(* Symbolic computation *)
Integrate[x^2 * Exp[-x^2], {x, -Infinity, Infinity}]
```

### Verilog
```verilog
module counter (
    input wire clk,
    input wire reset,
    input wire enable,
    output reg [7:0] count
);

always @(posedge clk or posedge reset) begin
    if (reset)
        count <= 8'b0;
    else if (enable)
        count <= count + 1;
end

endmodule
```

### VHDL
```vhdl
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

entity counter is
    Port ( clk : in STD_LOGIC;
           reset : in STD_LOGIC;
           enable : in STD_LOGIC;
           count : out STD_LOGIC_VECTOR (7 downto 0));
end counter;

architecture Behavioral of counter is
    signal count_int : unsigned(7 downto 0) := (others => '0');
begin
    process(clk, reset)
    begin
        if reset = '1' then
            count_int <= (others => '0');
        elsif rising_edge(clk) then
            if enable = '1' then
                count_int <= count_int + 1;
            end if;
        end if;
    end process;
    
    count <= std_logic_vector(count_int);
end Behavioral;
```

### Solidity
```solidity
pragma solidity ^0.8.0;

contract SimpleStorage {
    uint256 private storedData;
    address public owner;
    
    event DataChanged(uint256 newValue);
    
    modifier onlyOwner() {
        require(msg.sender == owner, "Not the owner");
        _;
    }
    
    constructor() {
        owner = msg.sender;
    }
    
    function set(uint256 x) public onlyOwner {
        storedData = x;
        emit DataChanged(x);
    }
    
    function get() public view returns (uint256) {
        return storedData;
    }
}
```

### Prolog
```prolog
% Facts
parent(tom, bob).
parent(tom, liz).
parent(bob, ann).
parent(bob, pat).
parent(pat, jim).

% Rules
grandparent(X, Z) :- parent(X, Y), parent(Y, Z).
ancestor(X, Z) :- parent(X, Z).
ancestor(X, Z) :- parent(X, Y), ancestor(Y, Z).

% Query examples:
% ?- grandparent(tom, ann).
% ?- ancestor(tom, jim).
```

This collection covers a wide range of programming languages, markup languages, configuration formats, and specialized domain-specific languages. Each code block demonstrates typical syntax and patterns for that particular language.
