// papers | source
// https://crypto.stanford.edu/pbc/notes/elliptic/explicit.html
// https://trustica.cz/2018/03/22/elliptic-curves-point-doubling/
// https://math.mit.edu/~dav/finitefields.pdf
// https://www.ams.org/journals/mcom/1985-44-170/S0025-5718-1985-0777280-6/S0025-5718-1985-0777280-6.pdf
// https://www.ams.org/journals/mcom/1987-49-179/S0025-5718-1987-0890272-3/S0025-5718-1987-0890272-3.pdf
// http://www.numdam.org/item/JTNB_1995__7_1_219_0.pdf
// https://core.ac.uk/download/pdf/10898289.pdf

// Surveys
//
// Modular arithmetic
// division over finite fields is not well defined, so we turn up doing inverse_mod_mult in order to 
// calc linear algebra on finite fields, for this reason, when calculating over finite fields, use
// modular arithmetic (inverse_mod in place of division).
//
// considering the field {0, 1, 2, 3}, what happens with 2/3?
//
// If gcd(k,N)=1 and ka ≡ kb(mod N), then a ≡ b (modN).
// If a and N are integers such that gcd(a,N)=1, then there exists an integer x such that ax ≡ 1 (modN).
//
// the inverse_mod of 'a' is some 'b' where 'a * b ≡ 1 (mod m)', e.g.:
// the inverse_mod of 3 (mod 5) -> 3 * b ≡ 1 (mod 5) -> 3 * 2 ≡ 6 ≡ 1 (mod 5)
//
// --------------------------
//
// Eliptic curves
// Weierstrass form -> y² = x³ + ax + b (short form, and easy arithmetic)
//
// the curve is non-singular if: 4a³ + 27b² != 0
// 
// inversion operation a.k.a. negation law (-)
// −O = O
// Let P1 = (u1, v1) ∈ EW. Then 1 = (u1, −v1 − a1u1 −a3)
// P1 + P2 = P2 + P1 (Commutativity)
// (P0 + P1) + P2 = P0 + (P1 + P2) (Associativity)

// TODO
// enforce finite field arithmetic with type system
// optimize point representation
// double-and-add

mod curve;

fn main() {
    let curve = curve::Curve{a:2, b:3, p:97};

    let g3 = curve.add(curve::Point::Pt{x:3, y:6}, curve::Point::Pt{x:3, y:6});

    dbg!("{}", &g3);

    let g4 = curve.double(g3);

    dbg!("{}", &g4);
}
