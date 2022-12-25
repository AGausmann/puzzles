use num_traits::PrimInt;

pub fn gcd<N: PrimInt>(a: N, b: N) -> N {
    if a % b == N::zero() {
        b
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<N: PrimInt>(a: N, b: N) -> N {
    a * b / gcd(a, b)
}
