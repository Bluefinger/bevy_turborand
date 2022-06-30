macro_rules! delegate_rng {
    ($method:tt, $type:tt, $doc:tt) => {
        #[doc = $doc]
        #[inline]
        pub fn $method(&mut self) -> $type {
            self.get_mut().$method()
        }
    };
    ($method:tt, $output:tt, $input:ty, $doc:tt) => {
        #[doc = $doc]
        #[inline]
        pub fn $method(&mut self, input: $input) -> $output {
            self.get_mut().$method(input)
        }
    };
}