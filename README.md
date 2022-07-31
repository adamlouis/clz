# clz

colorize text that matches a pattern

## usage

`clz <regex> [black|red|green|yellow|blue|magenta|cyan|white]`

## example

`cat README.md | clz 'u[a-zA-Z]+e' | clz 'c.?z' red | clz colorize yellow | clz text magenta | clz regex cyan`

![image of clz output in a terminal](/examples/clz.png?raw=true)
