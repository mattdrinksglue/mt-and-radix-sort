# SCUFFED RADIX SORT AND MT IMPLEMENTATION
Three versions bc I never settle #grindset
- regular (fast)
- base 16 only (fastest)
- base 2^n only (strangely slow like wtf)

## USAGE

``` rust
radix_sort(arr: Vec<u32>, base: u32);
radix_sort_16(arr: Vec<u32>);
radix_sort_bitmask(arr: Vec<u32>, number_of_bits_in_mask: u32);
```

## RUN

``` console
rustc radix_sort.rs && ./radix_sort
```

schizophrenia moment
