[![Actions Status](https://github.com/umgefahren/image-comp-lib-rust/workflows/Rust/badge.svg)](https://github.com/umgefahren/image-comp-lib-rust/actions)
![Lines of Code](https://github.com/umgefahren/image-comp-lib-rust/blob/main/cloc.svg)
[![Release Latest](https://badgen.net/github/release/umgefahren/image-comp-lib-rust)](https://github.com/umgefahren/image-comp-lib-rust/releases/tag/v0.0.1)
[![License](https://badgen.net/github/license/umgefahren/image-comp-lib-rust)](https://github.com/umgefahren/image-comp-lib-rust/blob/main/LICENSE)
[![Downloads](https://badgen.net/github/assets-dl/umgefahren/image-comp-lib-rust/v0.0.1)](https://github.com/umgefahren/image-comp-lib-rust/releases/tag/v0.0.1)


# Image Compression Algorithm

A new image compression algorithm.

In the newest version, the algorithm performs in most cases better then PNG. In fact the only image that get's bigger is img_4. The original img_2.png, for example is 12.8 MB; the compressed binary has only a size of 10.1 MB. However this is only achieved through aggressive settings on Zstd and takes quite some time to calculate.

## How the system works
### Clustering

The first step in the system is clustering the pixels. This happens in 5 Dimensions, with R, G, B, x and y of every Pixel. X & Y are normed over 255 in order to have a balance between the color values and the pixel position. This might offer a possible improvement.

In the current settings a [Kmeans](https://en.wikipedia.org/wiki/K-means_clustering) is used to define 3 dominant clusters. More clusters are possible, but the calculation time increases rapidly with an increasing number of clusters. The encoding supports up to 255 clusters, but this is probably overkill.

After defining the clusters, we calculate a cluster map, that removes the color values and just displays belonging to a cluster. A visualization of this would look like this:

![alt text](images/out.png)

### Grid

In the next step we lay a grid on top of the cluster map. The chunks of the grids are not fixed size. They vary in size near the edges. For every grid, we check if all pixels in a grid belong to the same cluster. If this is given, the pixel is calculated relative, otherwise absolute. The gird contains for every chunk a value that determines the cluster or that the chunk has to be calculated absolute. Here is an illustration of this grid map. Every white pixel, symbolizes an absolute chunk.

![alt_text](images/out_grid.png)

### Calculating Lists

In this step we, finally calculate the pixel values that are later written into the file. Every chunk is calculated according to the grid's perception of absolute or relative value. Every chucks pixel values are added to a super list of relative or absolute pixel values. The pixel values are calculated in wiggly lines. Every cluster has a minimum pixel value. This value is according to the minimum R, G, B value in that chunk. The resulting pixel value is an addition of this chunk value and the encoded pixel value.

### Flatten and Byte conversion

The grid, the cluster colors, the lines are converted in Vectors of u8 and then converted into bytes.

### ~~Deflate~~ Zstd

Grid and lines bytes representations are compressed with the ~~deflate~~ Zstd algorithm. This should achieve the compression and provides an opportunity to optimization.

### Write File

The resulting binary is just a list of the relevant compressed objects.

## How to use the shipped binariers

One should execute the binary with the arguments provided.
1. The first argument is 'dec' if this call should make compression.
2. The second argument is the input files Path.
3. The third argument is the output files Path.

## Advantages compared to PNG

Because of the grid, it's possible to load just specific chunks without loading the entire image, but this is not implemented yet. With further improvements it might be possible to surpass PNG in compression rate, but I can&#39;t prove that.

## Disadvantages compared to PNG

Because of the clusterisation it takes quite long to calculate a result. It might be possible to improve that, although this would probably require to abolish Kmeans for another clustering algorithm. One solution to that could be a neuronal net.

## Contribute
E-Mail me at hannes.furmans@web.de

## Acknowledgements
Special thanks to Dr. Artur Merke for inspiring and mentoring me.

## DISCLAIMER
As you can see, I'm not a computer scientist and not a very skilled programmer. I'm just a student taking careful steps in the world of computer science.
