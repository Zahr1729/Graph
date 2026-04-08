Modules

`utils` cannot assume any knowledge of any other module and so other modules may depend upon it.

`core` represents the minimum necessary data to construct an arbitrary graph.

All other modules may depend on `core`, but only upon `core` (and `utils`).