# Simple-Parse-Generator

Implementation of a simple parse generator that builds a LL(1) parser from a supplied grammar.
One current limitation is that the terminal and non terminal symbols currently
must consist of only one character. Meaningful error messages are also on the to do list


##Usage

```rust
  let set_builder = SetBuilder::new(&grammar);

  ...

  let parser = Parser::new(grammar, &set_builder).unwrap();
  parser.parse("input$")
```

If the supplied grammar does not a LL(1) gramamr Parser::new() will return an error;
If the input is not '$' terminated or is not derivable from the supplied grammar One
will get an error of the form:
```
  enum ParseError {
      In(InputErr),
      TableErr,
      StackErr,
  }
  
  enum InputErr {
      NotTerminated,
      Empty,
  }

```
