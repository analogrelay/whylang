# Specish

I don't want to obsess over a really long spec, but I do need to write some things down.

Why is designed (right now) to be a scripty-style language but with native compilation. Thus, it has scripty features.

## Structure

A Why program consists of a sequence of statements and declarations, concluding with a final expression. The value of the final expression is returned by writing it to stdout.

## High-level Grammar

```
Program := Statement* Expression

Statement := [TBD] ';'

Expression :=
    '' |
    [Number]
```