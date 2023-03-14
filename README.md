# Eliptic Curves Cryptography

**note**: This is not intended to be used in a real project.

A little survey that i've done while studying and implementing this repo:

## Modular arithmetic
- division over finite fields is not well defined, so we turn up doing inverse_mod_mult in order to calculate linear algebra on finite fields. For this reason, when calculating over finite fields, use modular arithmetic (inverse_mod in place of division).

`e.g. considering the field {0, 1, 2, 3}, what happens with 2/3? the field is not closed for this operation`

### gcd
- gcd(k,N)=1 and ka ≡ kb(mod N), then a ≡ b (modN).
- If a and N are integers such that gcd(a,N)=1, then there exists an integer x such that ax ≡ 1 (modN).

- the inverse_mod of 'a' is some 'b' where 'a * b ≡ 1 (mod m)', e.g.:
- the inverse_mod of 3 (mod 5) -> 3 * b ≡ 1 (mod 5) -> 3 * 2 ≡ 6 ≡ 1 (mod 5)

## Eliptic curves
**Weierstrass form** -> y² = x³ + ax + b (short form, and easy arithmetic)

- The curve is **non-singular** if: 4a³ + 27b² != 0
- Inversion operation a.k.a. negation law (-) -> -O = O


- Let P1 = (u1, v1) ∈ EW. Then 1 = (u1, −v1 − a1u1 −a3)
- P1 + P2 = P2 + P1 (Commutativity)
- (P0 + P1) + P2 = P0 + (P1 + P2) (Associativity)

## Enforcing modular arithmetic on Rust type system
- A recurrent aproach, is defining PrimeGroups type with Macros (macro_rules!), so you get compile-checking on fields, prohibiting diferent groups operating together

- Rust does not implement the "modulo" operator (check this: https://internals.rust-lang.org/t/mathematical-modulo-operator/5952)
