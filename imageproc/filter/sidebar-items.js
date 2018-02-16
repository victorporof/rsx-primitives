initSidebarItems({"fn":[["box_filter","Convolves an 8bpp grayscale image with a kernel of width (2 * `x_radius` + 1) and height (2 * `y_radius` + 1) whose entries are equal and sum to one. i.e. each output pixel is the unweighted mean of a rectangular region surrounding its corresponding input pixel. We handle locations where the kernel would extend past the image's boundary by treating the image as if its boundary pixels were repeated indefinitely."],["filter3x3","Returns 2d correlation of an image with a 3x3 row-major kernel. Intermediate calculations are performed at type K, and the results clamped to subpixel type S. Pads by continuity."],["gaussian_blur_f32","Blurs an image using a Gausian of standard deviation sigma. The kernel used has type f32 and all intermediate calculations are performed at this type."],["horizontal_filter","Returns horizontal correlations between an image and a 1d kernel. Pads by continuity. Intermediate calculations are performed at type K."],["median_filter","Applies a median filter of given `radius` to an image. Each output pixel is the median of the pixels in a `2 * radius + 1` square of pixels in the input image."],["separable_filter","Returns 2d correlation of view with the outer product of the 1d kernels `h_kernel` and `v_kernel`."],["separable_filter_equal","Returns 2d correlation of an image with the outer product of the 1d kernel filter with itself."],["vertical_filter","Returns horizontal correlations between an image and a 1d kernel. Pads by continuity."]],"struct":[["Kernel","A 2D kernel, used to filter images via convolution."]]});