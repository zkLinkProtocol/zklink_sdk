fn main() {
    #[cfg(feature = "python")]
    uniffi::uniffi_bindgen_main()
}
