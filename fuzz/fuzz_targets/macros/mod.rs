macro_rules! impl_arbitrary_test {
    {
        Mtid = $Mtid:ty,
        Int = $Int:ty,
    } => {
        fuzz_target!(|id: $Mtid| {
            validate!{
                id = id,
                Mtid = $Mtid,
                Int = $Int
            };
        });
    };
}

macro_rules! validate {
    {
        id = $id:ident,
        Mtid = $Mtid:ty,
        Int = $Int:ty
    } => {
        // validate size
        assert!($id >= <$Mtid>::NIL);
        assert!($id <= <$Mtid>::MAX);
        
        // validate string conversion
        assert_eq!($id, (&$id.to_string()).parse::<$Mtid>().unwrap());

        // validate integer conversion
        assert_eq!($id, <$Mtid>::try_from(<$Int>::from($id)).unwrap());
    };
}
