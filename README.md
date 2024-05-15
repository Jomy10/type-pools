# TypePools

Type pools are a data structure for storing values of multiple types.
Values can be queried from their type.

## An example

```rust
// Create a Type Pools structure
let mut pools = TypePools::new();

// Adding values
pools.push(1 as u32);
pools.push(2 as u32);
pools.push("Hello world");

// Query values
let int_pool = pools.type_pool::<u32>().unwrap();
let int_value: u32 = int_pool.values[0];
let string_value: &str = pools.get(0).unwrap();
```

## License

The library is licensed under the MIT license

