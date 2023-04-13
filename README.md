# image_transformer
A cli tool to apply a convolution matrix to a given image. 

Image an convolution matrix(kernel) are passed via cli arguments and the transformed image is saved in ./output/out.extension or a specified --ouput path
See the Usage section of this readme or the help command for more information.

# Usage
```cargo run -- --help```

Displays available parameters.
## Minimal
```cargo run```

This will produce a sample output based on a sample image contained in the images folder. The sample uses an edge detection kernel on a picture of L from Death Note.

## Custom
```cargo run -- ./images/cat.jpg --matrix=1,1,1-1```

## Arguments
### input_file
A path to the desired input file. Absolute path or relative to the crate root. No flag.
### matrix (-m, --matrix)
The convolution matrix to be used on the given image. Expects a comma separated list of numbers, e.g. -1,-1,-1,-1,8,-1,-1,-1,-1
### output_file (-o, --output)
The desired output location **without extension**. The extension will by default be the same as the input. The default location will be the output folder of this crate and a file called "out" with the same extension as the input file.

# Some sample convolution matrices

## Diagonal gradient effect
```1,1,1,-1```
## Sharpening effect
```0,-1,0,-1,5,-1,0,-1,0```
## Edge detection
```-1,-1,-1,-1,8,-1,-1,-1,-1```

## Subtle blur
```1,1,1,1,1,2,2,1,1,2,2,1,1,1,1,1```

## Laplacian filter, sharpen
```0,-1,-1,-1,0,-1,0,0,0,-1,-1,0,16,0,-1,-1,0,0,0,-1,0,-1,-1,-1,0```

## Gaussian filter, blur
```-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,49,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1,-1```