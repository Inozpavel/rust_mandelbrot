# How to use
```bash
Usage: mandelbrot.exe [OPTIONS] [HEIGHT] [WIDTH]

Arguments:
[HEIGHT]  image height [default: 1920]
[WIDTH]   image width [default: 1080]

Options:
--num-threads <NUM_THREADS>  Num threads to speed up computation. Default to std::thread::available_parallelism()
--upper-left <UPPER_LEFT>    Upper left complex point for computation # Example --upper-left='-1.2+0.35i' [default: -1.6+0.05i]
--lower-right <LOWER_RIGHT>  Lower right complex point for computation # Example --lower-right='-1+0.2i' [default: -1.8-0.05i]
-h, --help                   Print help
```

Example output:

![image](./images/result-5000x5000.png)