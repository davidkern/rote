/// A 128-bit word
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(align(16))]
pub struct Word128 {
    data: u128,
}

impl Word128 {
    pub fn overflowing_add(self, rhs: Word128) -> (Self, bool) {
        let (data, carry) = self.data.overflowing_add(rhs.data);

        (Self { data }, carry)
    }

    pub fn overflowing_sub(self, rhs: Word128) -> (Self, bool) {
        let (data, carry) = self.data.overflowing_sub(rhs.data);

        (Self { data }, carry)
    }
}

impl<T> From<T> for Word128
where T: Into<u128>
{
    fn from(val: T) -> Self { Self { data: val.into() }}
}

impl std::ops::Add for Word128 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            data: self.data + other.data
        }
    }
}

impl std::ops::AddAssign for Word128 {
    fn add_assign(&mut self, other: Self) {
        self.data += other.data
    }
}

impl std::ops::Div for Word128 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            data: self.data / other.data
        }
    }
}

impl std::ops::DivAssign for Word128 {
    fn div_assign(&mut self, other: Self) {
        self.data /= other.data;
    }
}

impl std::ops::Mul for Word128 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            data: self.data * other.data
        }
    }
}

impl std::ops::MulAssign for Word128 {
    fn mul_assign(&mut self, other: Self) {
        self.data *= other.data;
    }
}

impl std::ops::Rem for Word128 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {
            data: self.data % other.data
        }
    }
}

impl std::ops::RemAssign for Word128 {
    fn rem_assign(&mut self, other: Self) {
        self.data %= other.data
    }
}

impl std::ops::Sub for Word128 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            data: self.data - other.data
        }
    }
}

impl std::ops::SubAssign for Word128 {
    fn sub_assign(&mut self, other: Self) {
        self.data -= other.data
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_conversion() {
        Word128::from(0u8);
        Word128::from(0u16);
        Word128::from(0u32);
        Word128::from(0u64);
        Word128::from(0u128);
    }
}