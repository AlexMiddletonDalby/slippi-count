# slippi-count
Pure Rust cli app  based on `peppi.rs` which can be used to count various events/metrics from a folder of .slp files. It makes use of `threadpool` to parallelize this to speed things up with large slippi file libraries. At the moment, it is only able to total up the amount of in-game time for a directory of slippi files, filtered by tags provided to it.

I may get round to adding more options (total wins, losses, L-cancels, uses of specific moves etc) but right now this is more of just a proof of concept.
