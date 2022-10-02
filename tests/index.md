# Tests

## Pixels encoding

There are no need for 3-length and 4-length pixels, because they are just a combination of 2-length instructions. 
For sanity and encoding correctness, 6-length pixels are tested.

``` rgb rgba run ```
 
```
rgb-run run-rgb
rgba-run run-rgba
rgb-rgba rgba-rgb
rgb-index rgba-index
rgb-luma rgba-luma
rgb-diff rgba-diff
```

```
rgb-rgba-run-diff-index-luma
run-luma-rgb-index-diff-rgba
```