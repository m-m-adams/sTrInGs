
# sTrInGs
This is an incredibly useful utility to display your level of happiness with strings output.

    USAGE:  
        sTrInGs [OPTIONS] <in-path>

    FLAGS:  
        -h, --help       Prints help information  
        -V, --version    Prints version information  

    OPTIONS:  
        -e, --encoding     Select character size and endianness: 
                           s = 7-bit, S = 8-bit, {b,l} = 16-bit, {B,L} = 32-bit [default: S]
        -n, --bytes        minimum length to search for [default: 4]  
        -p, --swap-prob    probability of swapping case [default: 0.5]  

    ARGS:  
        in-path    The path to read from
