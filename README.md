# brain_rust
A BF transpiler to rust written as a rust declarative macro

To use, invoke the brain_rust! macro and input a brainfuck program's source. Tokens other than: + - < > , . [ ] are not allowed, so you may need to strip those out.
