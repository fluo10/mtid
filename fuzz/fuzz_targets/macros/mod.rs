macro_rules! impl_arbitrary_test {
    {
        caretta_id = $caretta_id:ty,
        Int = $Int:ty,
    } => {
        fuzz_target!(|id: $caretta_id| {
            validate!{
                id = id,
                caretta_id = $caretta_id,
                Int = $Int
            };
        });
    };
}

macro_rules! validate {
    {
        id = $id:ident,
        caretta_id = $caretta_id:ty,
        Int = $Int:ty
    } => {
        // validate size
        assert!($id >= <$caretta_id>::NIL);
        assert!($id <= <$caretta_id>::MAX);
        
        // validate string conversion
        assert_eq!($id, (&$id.to_string()).parse::<$caretta_id>().unwrap());

        // validate integer conversion
        assert_eq!($id, <$caretta_id>::try_from(<$Int>::from($id)).unwrap());
    };
}
