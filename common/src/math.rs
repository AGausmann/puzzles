use glam::IVec2;
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

pub struct IRect {
    pub lower: IVec2,
    pub upper: IVec2,
}

impl IRect {
    pub fn contains(&self, v: IVec2) -> bool {
        (v.cmpge(self.lower) & v.cmple(self.upper)).all()
    }
}
