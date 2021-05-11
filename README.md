# 2D Landscape Generation

This is a simple 2D procedural landscape generator command line application.

## Example
![example 1](./screenshots/default_128.png)

*Generated with `./landscape-gen-2d.exe -s 128`*

## Usage

```
./landscape-gen-2d.exe [options]
```

There are several options available:
- `-s` or `--size`: Sets the height and width of the output image.
- `-f` or `--flatness`: Sets the **Flatness Factor** of the landscape.
- `-v` or `--variance`: Sets the **Variance Factor** of the landscape.

## Terminology
### Flatness Factor

The **Flatness Factor** is a decimal number that is used to control how likely it is that flat sections will occur in your landscape.

Values for this parameter can be in the range `[0.0, inf)`. Where `0.0` means that there will be no flat sections of your landscape, and increasingly higher values will result in an increasingly higher probability of flat sections occuring in the landscape.

### Variance Factor

The **Variance Factor** is a positive integer that represents the maximum difference in terrain height for adjacent blocks.

Values for this parameter can be in the range `[0, inf)`. Note that `0.0` should cause your landscape to be *deterministically* flat. (though float inaccuracy may prove otherwise in extreme cases)
