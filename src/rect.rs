use num_traits;
use partial_cmp;
use std::cmp;
use std::ops;

pub trait Rect<T> {
    fn y_min(&self) -> T;
    fn x_min(&self) -> T;
    fn y_max(&self) -> T;
    fn x_max(&self) -> T;

    fn height(&self) -> T
        where T: ops::Sub<Output = T>
    {
        self.y_max() - self.y_min()
    }

    fn width(&self) -> T
        where T: ops::Sub<Output = T>
    {
        self.x_max() - self.x_min()
    }

    fn area(&self) -> T
        where T: cmp::PartialOrd + num_traits::Zero + ops::Mul<Output = T> + ops::Sub<Output = T>
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
        T: Clone + cmp::PartialOrd + num_traits::Zero + ops::Div<Output = T> + ops::Mul<Output = T> + ops::Sub<Output = T>,
        RHS: Rect<T>
    {
        (|| {
            let y_min = partial_cmp::max(self.y_min(), rhs.y_min())?;
            let x_min = partial_cmp::max(self.x_min(), rhs.x_min())?;
            let y_max = partial_cmp::min(self.y_max(), rhs.y_max())?;
            let x_max = partial_cmp::min(self.x_max(), rhs.x_max())?;

            let h = y_max - y_min;
            let w = x_max - x_min;
            let i = if h > T::zero() && w > T::zero() {
                h * w
            } else {
                T::zero()
            };

            Some(i.clone() / (self.area() + rhs.area() - i))
        })()
                .unwrap_or(T::zero())
    }
}
