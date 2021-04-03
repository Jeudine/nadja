use crate::interface::TChannel;

pub trait Channel<T: TChannel> {
    fn read(&self) -> T;
}
