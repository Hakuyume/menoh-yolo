use num_traits;
use partial_cmp;
use std::cmp;
use std::ops;

pub trait Rect<T> {
    fn top(&self) -> T;
    fn left(&self) -> T;
    fn bottom(&self) -> T;
    fn right(&self) -> T;

    fn height(&self) -> T
    where
        T: ops::Sub<Output = T>,
    {
        self.bottom() - self.top()
    }

    fn width(&self) -> T
    where
        T: ops::Sub<Output = T>,
    {
        self.right() - self.left()
    }

    fn area(&self) -> T
    where
        T: cmp::PartialOrd + num_traits::Zero + ops::Mul<Output = T> + ops::Sub<Output = T>,
    {
        let h = self.height();
        let w = self.width();
        if h > T::zero() && w > T::zero() {
            h * w
        } else {
            T::zero()
        }
    }

    fn iou<RHS>(&self, rhs: &RHS) -> T
    where
        T: Clone
            + cmp::PartialOrd
            + num_traits::Zero
            + ops::Div<Output = T>
            + ops::Mul<Output = T>
            + ops::Sub<Output = T>,
        RHS: Rect<T>,
    {
        (|| {
            let top = partial_cmp::max(self.top(), rhs.top())?;
            let left = partial_cmp::max(self.left(), rhs.left())?;
            let bottom = partial_cmp::min(self.bottom(), rhs.bottom())?;
            let right = partial_cmp::min(self.right(), rhs.right())?;

            let h = bottom - top;
            let w = right - left;
            let i = if h > T::zero() && w > T::zero() {
                h * w
            } else {
                T::zero()
            };

            Some(i.clone() / (self.area() + rhs.area() - i))
        })().unwrap_or(T::zero())
    }
}
