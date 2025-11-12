macro_rules! impl_tests {
    (
        Self = $SelfT:ty,
        Uint = $Uint:ty,
    ) => {
        fn validate_string_convertion(value: $SelfT) -> Result<bool, mtid::Error> {
            Ok(value == value.to_string().parse::<$SelfT>()?)
        }

        fn validate_integer_conversion(value: $SelfT) -> Result<bool, mtid::Error> {
            Ok(value == <$SelfT>::try_from(<$Uint>::from(value))?)
        }
        fn validate_bytes_conversion(value: $SelfT) -> bool {
            value == <$SelfT>::from_bytes_lossy(&value.to_bytes())
        }

        #[test]
        fn nil_string_convertion() {
            assert!(validate_string_convertion(<$SelfT>::NIL).unwrap());
        }
        #[test]
        fn nil_integer_conversion() {
            assert!(validate_integer_conversion(<$SelfT>::NIL).unwrap());
        }

        #[test]
        fn max_string_convertion() {
            assert!(validate_string_convertion(<$SelfT>::MAX).unwrap());
        }
        #[test]
        fn max_integer_conversion() {
            assert!(validate_integer_conversion(<$SelfT>::MAX).unwrap());
        }
        #[test]
        fn nil_bytes_convertion() {
            assert!(validate_bytes_conversion(<$SelfT>::NIL));
        }
        #[test]
        fn max_bytes_convertion() {
            assert!(validate_bytes_conversion(<$SelfT>::MAX));
        }
        

        #[test]
        fn boundary_value() {
            let _ = <$SelfT>::try_from(<$SelfT>::CAPACITY - 1).unwrap();
            let _ = <$SelfT>::try_from(<$SelfT>::CAPACITY).unwrap_err();
        }

        #[test]
        #[cfg(feature = "rand")]
        fn random() {
            for _ in 0..10 {
                let id: $SelfT = rand::random();
                assert!(<$SelfT>::NIL < id);
                assert!(<$SelfT>::MAX >= id);
            }
        }

        #[test]
        #[cfg(feature = "rand")]
        fn random_int() {
            let mut rng = rand::rng();
            for _ in 0..10 {
                let value: $Uint = rng.random_range(1..<$SelfT>::CAPACITY);
                let _ = <$SelfT>::try_from(value).unwrap();
            }
        }

        #[test]
        #[cfg(feature = "rand")]
        fn oversized_random_int() {
            let mut rng = rand::rng();
            for _ in 0..10 {
                let value: $Uint = rng.random_range(<$SelfT>::CAPACITY..<$Uint>::MAX);
                let _ = <$SelfT>::try_from(value).unwrap_err();
            }
        }

        #[test]
        fn partial_ord() {
            assert!(<$SelfT>::NIL < <$SelfT>::MAX);
        }

        #[test]
        fn sort() {
            let mut vec = vec![<$SelfT>::MAX, <$SelfT>::NIL];
            vec.sort();
            assert_eq!(vec[0], <$SelfT>::NIL);
            assert_eq!(vec[1], <$SelfT>::MAX);
        }
    };
}
