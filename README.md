# Tremor
A sofware rasteriser/renderer and and Quake level viewer written in Rust. Please note this project is still under heavy development as of yet is not feature complete, most notably missing texture rendering.

The following are example images taken from the first level of Quake.
![Image 1](/images/image.PNG)
![Image 2](/images/image2.PNG)

The following is a brief description of each project and what it is used for.

tremor
> The entry point of the application, used to initialise other components.

lib_tremor_common
> Holds utilities like BSP file parsing.

lib_revert
> A software renderer used to render 3D images to an in-memory bitmap via CPU rasterisation, and where most of the attention has been paid for this project so far.

lib_foundry
> The three lib_foundry projects are used to provide a platform independant layer, to easily allow for  future platforms such as Linux & macOS