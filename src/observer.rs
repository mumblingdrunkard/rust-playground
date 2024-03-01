pub trait Observer<T1> {
    fn invoke(&self, arg1: T1);
}
