# EHTML

EHTML is an HTML validator and minimalistic preprocessor. It validates the basic structure (meaning doctype, html, head, title and body tags in correct order) of an HTML file passed as an argument.
It also validates some basic SEO best practices.
It has a simple template engine that can handle variables.

## Working with the engine

To declare a variable, use the following syntax: 

`<let name(:type)?=val>`

Al varaiables work in a global inmutable way and they should be declare at the top of the file before the DOCTYPE tag (so they are hoisted by sintax).
You can optionaly type check you variables by adding ":" and the type in front of the name.
The suppoerted types are:
- int
- float
- bool
- char
- str

At the end of all the variable declarations and before the DOCTYPE tag there should be a `<endvars/>`, for example:

```
<let name:str = "Pedro">
... other variable declarations ...
<endvars/>
<!DOCTYPE html>
...
```

To insert a variable in the document, use a pp tag like this:

`<pp name>`

## Runing

`cargo run <html_file>`

## ToDo
- [x] Validate html structure (doctype, html, head, title, body) 
- [x] SEO tips (lenght of title and description, images format, alt and lazy load)
- [x] html simple variables (all vars global, declared before de doctype tag, and hoisted (hoisting))
- [ ] templates (some sort of macros/functions?, params would be declared, used and processed totaly diferent and separate from vars)
### Possible extension to support css and js
- [ ] css optimization
- [ ] check for bad practices like the use of eval or innetHTML
- [ ] compile jquery to js
### I don't intend on replacing PHP or JS but is still a maybe
- [ ] html simple conditional
- [ ] html loop
