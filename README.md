# How to use
```bash
Usage: mandelbrot.exe [OPTIONS] [HEIGHT] [WIDTH]

Arguments:
[HEIGHT]  image height [default: 600]
[WIDTH]   image width [default: 600]

Options:
--num-threads <NUM_THREADS>
--upper-left <UPPER_LEFT>    upper left complex point for computation # Example --upper-left='-1.2+0.35i' [default: -1.6+0.05i]
--lower-right <LOWER_RIGHT>  lower right complex point for computation # Example --lower-right='-1+0.2i' [default: -1.8-0.05i]
-h, --help                   Print help
```

Example output:

![image](./images/result-5000x5000.png)