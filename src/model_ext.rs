use menoh;
use ndarray;

pub trait ModelExt {
    fn get_view<'a>(&'a self, name: &str) -> Result<ndarray::ArrayViewD<'a, f32>, menoh::Error>;
    fn get_view_mut<'a>(
        &'a mut self,
        name: &str,
    ) -> Result<ndarray::ArrayViewMutD<'a, f32>, menoh::Error>;
}

impl ModelExt for menoh::Model {
    fn get_view<'a>(&'a self, name: &str) -> Result<ndarray::ArrayViewD<'a, f32>, menoh::Error> {
        let (dims, buf) = self.get_variable(name)?;
        Ok(ndarray::ArrayViewD::from_shape(dims, buf).unwrap())
    }

    fn get_view_mut<'a>(
        &'a mut self,
        name: &str,
    ) -> Result<ndarray::ArrayViewMutD<'a, f32>, menoh::Error> {
        let (dims, buf) = self.get_variable_mut(name)?;
        Ok(ndarray::ArrayViewMutD::from_shape(dims, buf).unwrap())
    }
}
