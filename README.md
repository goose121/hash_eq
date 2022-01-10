# hash_eq

`hash_eq` is a library which provides the trait `HashEq` for comparing
the parts of values that will be hashed. This also allows using types
which cannot be borrowed from a `HashMap`'s key type but which still
have an equivalent hash implementation to be used as `HashMap` keys.