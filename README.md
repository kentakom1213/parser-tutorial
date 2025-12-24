# parser-tutorial

```
expr    = term (("+" | "-") term)*
term    = unary (("*" | "/") unary)*
unary   = ("+" | "-") unary | power
power   = postfix ("^" power)?
postfix = primary
primary = number
        | ident "(" expr ")"
        | "(" expr ")"
```
