# Rust Life
This is a very simple implementation of the Conway's Life cellular automaton. It's quite buggy, I believe, because I've been using it as a playground to master the Rust's language basic features. Still, it generates some reasonable results.

At the beginning, I've been thinking about adding a GUI of some sort to display an animated field of cells. But it appeared to be too time-consuming for the educational project, plus there was no standard GUI for the Rust. So, wasting time for studying some peculiar implementations was really not worth it. In any case, I'm not going to use the Rust for some user-oriented programming. So I stuck to the two IO "devices": the console and files.

The console is used for output only. There is no way to setup initial configuration using it. While the files may be used for both input and output. There are two formats supported: the [plaintext](https://conwaylife.com/wiki/Plaintext) and the [run length encoded](https://conwaylife.com/wiki/Run_Length_Encoded). For the output only, the venerable [GIF](https://en.wikipedia.org/wiki/GIF) format may be used as well. Though the current program does not utilize its animation features, it just writes a bunch of files to the disk, it is still much more perceivable than the text files mentioned earlier.

A few words to say about the edge conditions. The behavior of the patterns found at the very edge of the field greatly depends on what is beyond the edge (I refer to it as "fence" in code). There are a few different approaches to address it and current implementation presents the following options:
* cliff;
* warp
* fade away;

The "cliff" means that the field is surrounded by permanently dead cells. The patterns in this case meet maximal distortions. The "warp" connects the opposing sides of the fields, so a moving pattern, like a glider, crossing the left edge immediately appears from the right. The "fade away" is a certain compromise between these to extremes. In this scenario, the active field is put into the center of another, slightly bigger one. Beyond the latter the cells are permanently dead, like in the "cliff" case. The cells between the active and big fields are invisible to the user, but are taken into account when updating the generation. From the user's perspective that means that the glider moving from left to right crosses the right edge and disappears with no traces. It eventually reaches the edge of the big field, gets distorted and finally dies, but the user never sees it.

The program is fully command-line oriented. To get the complete list of the supported switches simply invoke it with "-h".


