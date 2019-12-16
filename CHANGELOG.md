# 0.1.2

-   Changed Feistel network rounds from 4 to 12 (and later will make this configurable).
    -   Randomness on small max_values was very poor at 4 rounds, so just increased it to 12 for now.
-   Have the first automated randomness test, that compares chi squared values against a "real randomness" shuffler.
    -   Not the greatest test of randomness but it's a start.
-   Better Travis continuous integration config.

# 0.1.1

-   Documentation updates.
-   Update `Cargo.toml` to better describe project.

# 0.1.0

-   Initial release.
-   Unstable, will almost certainly change API.
