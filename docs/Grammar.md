# Grammar

Symbols in `ALLCAPS` are terminals defined in the Tokens list below. Symbols in `PascalCase` are non-terminals defined by other grammar rules.

```
Document := 
	Statement |
	Statement Document

Statement := Expression

Expression :=
	ConstantExpression |
	CallExpression

ConstantExpression :=
	INTEGER |
	STRING

CallExpression :=
	IDENTIFIER LPAREN ArgumentList RPAREN

ArgumentList :=
	Expression |
	Expression COMMA ArgumentList

```

## Tokens

```
INTEGER := \d+
STRING	:= "[^"]+"
LPAREN	:= '('
RPAREN	:= ')'
COMMA	:= ','
```
