use serde_valid::Validate;

mod issue54 {
    use super::*;

    #[test]
    fn test_enum_valians_works() {
        #[derive(Validate)]
        enum Works {
            VariantB(),
            VariantA,
        }

        assert!(Works::VariantA.validate().is_ok());
        assert!(Works::VariantB().validate().is_ok());
    }

    #[test]
    fn test_enum_valiant_fied_case() {
        #[derive(Validate)]
        enum Fails {
            VariantA,
            VariantB(),
        }

        assert!(Fails::VariantA.validate().is_ok());
        assert!(Fails::VariantB().validate().is_ok());
    }
}
