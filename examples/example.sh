#!/bin/bash

cat README.md | clz 'u[a-zA-Z]+e' | clz 'c.?z' red | clz colorize yellow | clz text magenta | clz regex cyan
