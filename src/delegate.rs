macro_rules! delegate_rng_trait {
    ($method:tt, $type:ty, $doc:tt) => {
        #[doc = $doc]
        #[inline]
        fn $method(&mut self) -> $type {
            self.get_mut().$method()
        }
    };
    ($method:tt, $output:ty, $input:ty, $doc:tt) => {
        #[doc = $doc]
        #[inline]
        fn $method(&mut self, input: $input) -> $output {
            self.get_mut().$method(input)
        }
    };
}
