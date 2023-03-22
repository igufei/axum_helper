use self::{fail::Fail, success::Success};
pub mod fail;
pub mod success;
pub type Response<T> = Result<Success<T>, Fail>;