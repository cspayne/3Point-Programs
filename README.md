# 3Point Programs

## Description
This project consists of a parser that analyzes programs according to the grammar rules of an invented language '3Point', which portrays 3 named variables assigned to 2d points. If the 3Point program has no errors, the parser will output translations of the 3Point program that can be run by one of the other 2 programs included in the project. 

The project was designed and prompted by Dr. Carlos Arias, and programmed by me, Cypress Payne, for Concepts in Programming Languages (CSC3310)

# 3Point Parser - Rust

## Description
This is a program in Rust that takes a program written in 3Point, and outputs:
1. If the program has lexical or syntax errors, the error that was found. Use panic version of error handling.
2. If the program is OK, depending on a command line flag the program will produce:
   1.	If the flag is -s the program will output a function call in Scheme that is going to be called by a program in Scheme that will calculate properties of those three points.
   2. If the flag is -p the program will output a series of queries about those three points.

The program should run like this:
```
prompt>cargo run input.txt -s
; Processing Input File input.txt
; Lexical and Syntax analysis passed
(calculate-triangle (make-point 2 3) (make-point 1 4) (make-point 3 4))
prompt>
```

## Grammar

```
START     --> POINT_DEF; POINT_DEF; POINT_DEF.
POINT_DEF --> ID = point(NUM, NUM)
ID        --> LETTER+
NUM       --> DIGIT+
LETTER    --> a | b | c | d | e | f | g | ... | z
DIGIT     --> 0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9

```

The tokens of this grammar are:
```
POINT
ID
NUM
SEMICOLON
COMMA
PERIOD
LPAREN
RPAREN
ASSIGN
```

Given the following program written in this language:
```
a = point(2, 3);
b = point(1, 1);
c = point(1, 3).
```
The tokens that it would generate are:
1. `ID  a`
1. `ASSIGN`
1. `POINT`
1. `LPAREN`
1. `NUM 2`
1. `COMMA`
1. `NUM 3`
1. `RPAREN`
1. `SEMICOLON`
1. `ID  b`
1. `ASSIGN`
1. `POINT`
1. `LPAREN`
1. `NUM 1`
1. `COMMA`
1. `NUM 1`
1. `RPAREN`
1. `SEMICOLON`
1. `ID  c`
1. `ASSIGN`
1. `POINT`
1. `LPAREN`
1. `NUM 1`
1. `COMMA`
1. `NUM 3`
1. `RPAREN`
1. `PERIOD`

Notice that the ID and NUM tokens have their lexeme associated. Also notice that in the language the elements do not need to be separated by space.

## How to run the program

### Scheme Output
To generate scheme output you will add the `-s` flag at the end of the command:
```
prompt> cargo run input.txt -s
; processing input file input.txt
; Lexical and Syntax analysis passed
(calculate-triangle (make-point 2 3) (make-point 1 1) (make-point 1 3))
```

### Prolog Output
To generate scheme output you will add the `-p` flag at the end of the command:
```
prompt> cargo run input.txt -p
/* processing input file input.txt
   Lexical and Syntax analysis passed */
query(line(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(triangle(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(vertical(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(horizontal(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(equilateral(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(isosceles(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(right(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(scalene(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(acute(point2d(2,3), point2d(1,1), point2d(1, 3)))
query(obtuse(point2d(2,3), point2d(1,1), point2d(1, 3)))
writeln(T) :- write(T), nl.
main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),
      halt.

```
## Files
- `src/main.rs` Source code in Rust for your lexical and syntax analysis
- `test0.cpl`, `test1.cpl`, `test2.cpl`, `test3.cpl`, `test4.cpl`, the test files provided for you to test. Notice that `test3.cpl` has a lexical error and `test4.cpl` has a syntax error.

Additional files with decorated (Tokens with Lexemes on the Leafs) parse trees. The file is named like the input file but with the extension `.pt` (stands for parse tree), for instance if the input file is `test1.cpl` the parse tree should be in file `test1.pt`.

# 3Point Questionairre - Prolog

## Description
This is a Prolog program that stores information about geometric objects. 
The objects constructed are:
-	2D Point
-	Triangle
The program is able to answer the following questions:

English | Prolog
------- | ------
Is the line defined by two points vertical? |	`vertical(point2d(x,y), point2d(x,y))`
Is the line defined by two points horizontal? |	`horizontal(point2d(x,y), point2d(x,y))`
Do the given three points define a line? |	`line(point2d(x,y), point2d(x,y), point2d(x,y))`
Do the given three points define a triangle? | `triangle(point2d(x,y), point2d(x,y), point2d(x,y))`
Is the triangle isosceles? |	`isosceles(point2d(x,y), point2d(x,y), point2d(x,y))`
Is the triangle equilateral? |	`equilateral(point2d(x,y), point2d(x,y), point2d(x,y))`
Is the triangle right? |	`right(point2d(x,y), point2d(x,y), point2d(x,y))`
Is the triangle scalene? |	`scalene(point2d(x,y), point2d(x,y), point2d(x,y))`
Is the triangle acute? |	`acute(point2d(x,y), point2d(x,y), point2d(x,y))`
Is the triangle obtuse?	| `obtuse(point2d(x,y), point2d(x,y), point2d(x,y))`

## Files
`threepoints.pl` 

## Integration with Rust Parser
Steps to integrate the parser made in Rust:
1. Create a 3Points Language "program" call it `test.cpl`, define three points.
1. Run your parser: `cargo run test.cpl -p > mytest.pl`
1. Paste your generated test to your prolog program: `cat threepoints.pl mytest.pl > full.pl`
1. Run your program to check what those three points in `test.cpl` represent: `swipl -q -f full.pl -t main`

# 3Point Calculations - Scheme

## Description
This is a Scheme program that implements the following functions:
- `(make-point x-cor y-cor)`. This function creates a “list” that will have two elements: `x-cor` and `y-cor`. 
- `(is-line point1 point2 point3)`. Using the created points (using `make-point`) this function returns true (`#t`) if the three points form a line or false (`#f`) if they form a triangle.
- `(distance point1 point2)`. Calculates the distance between two points
- `(perimeter point1 point2 point3)`. Calculates the perimeter of a triangle defined by the three points
- `(area point1 point2 point3)`. Calculates the area of a triangle defined by the three points
- `(calculate-triangle point1 point2 point3)`. Calculates the perimeter, area and interior angles of the triangle formed by the three points.

## Files
`threepoints.scm`

## Integration with Rust Parser
Steps to integrate the parser made in Rust:
1. Create a 3Points Language "program" call it `test.cpl`, define three points.
1. Run your parser: `cargo run test.cpl -s > mytest.scm`
1. Paste your generated test to your scheme program: `cat threepoints.scm mytest.scm > full.scm`
1. Run your program to check what those three points in `test.cpl` represent: `scheme --load full.scm`
