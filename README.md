# EHTML

EHTML is a html validator. (For now) It validates the basic structure (mening doctype, html, head, title and body tags in correct order) of an HTML file passed as an argument.
It also validates some basic SEO good practices.

## Runing

`cargo run <html_file>`

## ToDo
- [x] Validate html structure (doctype, html, head, title, body) 
- [x] SEO tips (lenght of title and description, images format, alt and lazy load)
- [ ] html simple variables (all vars global, declared before de doctype tag, and hoisted (hoisting))
- [ ] templates (some sort of macros/functions?, params would be declared, used and processed totaly diferent and separate from vars)
### Possible extension to support css and js
- [ ] css optimization
- [ ] check for bad practices like the use of eval or innetHTML
- [ ] compile jquery to js
### I don't intend on replacing PHP or JS but is still a maybe
- [ ] html simple conditional
- [ ] html loop
