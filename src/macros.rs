macro_rules! create_opaque_type {
    ($type_name:ident) => {
        #[cfg(feature = "nightly")]
        extern "C" {
            pub type $type_name;
        }

        #[cfg(not(feature = "nightly"))]
        #[repr(C)]
        pub struct $type_name {
            _private: [u8; 0],
        }
    };
}
