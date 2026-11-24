# `/embedded`

This directory contains a cargo-workspace for each targeted architecture.

This structure aims to simplify the build procedure and IDE support, as cargo doesn't handle mixed targets in a single workspace well. As soon as there's a better solution the crates might get be re-grouped to better reflect the project architecture.

The extra-level of `embedded` isn't technically required, but helps to de-clutter the project's root directory.
