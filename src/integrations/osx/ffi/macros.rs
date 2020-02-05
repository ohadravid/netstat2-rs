// #define ROUNDUP64(a)                                                           \
//     ((a) > 0 ? (1 + (((a)-1) | (sizeof(uint64_t) - 1))) : sizeof(uint64_t))

macro_rules! ROUNDUP64 {
    ($a: expr) => {
        if $a > 0 {
            1 + (($a as usize - 1) | (std::mem::size_of::<uint64_t>() - 1)) as isize
        } else {
            std::mem::size_of::<uint64_t>() as isize
        }
    };
}
