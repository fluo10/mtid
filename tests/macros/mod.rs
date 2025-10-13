macro_rules! impl_tests {
    (
        Self = $SelfT:ident,
        Integer = $Integer:ident,
    ) => {

        fn validate_string_convertion(value: $SelfT) -> Result<bool, mtid::Error> {
            Ok(value == value.to_string().parse::<$SelfT>()?)
        }
        
        fn validate_integer_conversion(value: $SelfT) -> Result<bool, mtid::Error> {
            Ok(value == $SelfT::try_from($Integer::from(value))?)
        }

        #[test]
        fn nil_string_convertion() {
            assert!(validate_string_convertion($SelfT::NIL).unwrap());
        }
        #[test]
        fn nil_integer_conversion() {
            assert!(validate_integer_conversion($SelfT::NIL).unwrap());
        }

        #[test]
        fn max_string_convertion() {
            assert!(validate_string_convertion($SelfT::MAX).unwrap());
        }
        #[test]
        fn max_integer_conversion() {
            assert!(validate_integer_conversion($SelfT::MAX).unwrap());
        }
        

        #[test]
        fn boundary_value() {
            let _ = $SelfT::try_from($SelfT::CAPACITY-1).unwrap();
            let _ = $SelfT::try_from($SelfT::CAPACITY).unwrap_err();
        }

        #[test]
        fn random_int() {
            let mut rng = rand::thread_rng();
            for _ in 0..10 {
                let id: $SelfT = rng.r#gen();
                assert!($SelfT::NIL < id);
                assert!($SelfT::MAX >= id);
            }
        }

        #[test]
        fn oversized_random_int() {
            let mut rng = rand::thread_rng();
            let _ = $SelfT::try_from(0).unwrap();
            for _ in 0..10 {
                let value: $Integer = rng.gen_range($SelfT::CAPACITY..$Integer::MAX);
                let _ = $SelfT::try_from(value).unwrap_err();
            }
        }

        #[test]
        fn partial_ord() {
            assert!($SelfT::NIL < $SelfT::MAX);
        }

        #[test]
        fn sort() {
            let mut vec = vec![$SelfT::MAX, $SelfT::NIL];
            vec.sort();
            assert_eq!(vec[0], $SelfT::NIL);
            assert_eq!(vec[1], $SelfT::MAX);
        }
    }
}