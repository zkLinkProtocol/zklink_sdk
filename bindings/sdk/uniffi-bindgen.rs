fn main() {
    #[cfg(any(feature = "python", feature = "kotlin"))]
    uniffi::uniffi_bindgen_main()
}
