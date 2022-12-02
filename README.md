# Nazm

---

Root
ن ظ م • (n-ẓ-m)

forms words related to organizing, organization, arrangement, orderliness or composition[^1]

[^1]: [Wiktionary](https://en.wiktionary.org/wiki/%D9%86_%D8%B8_%D9%85)

---

`nazm` is an attempt at defining a declarative configuration format, and creating a declarative configuration export,
diff and application tool for Windows 11.

This project is currently in very early development and looking for contributors. Please do not open issues with feature
requests as an end-user at this point.

Presently, the primary focus is on gathering a complete list of configuration settings that require a single registry
change.

If you are interested in contributing, drop by the `#development` channel on
the [Komorebi Community Discord Server](https://discord.gg/mGkn66PHkx).

Contribution standards and expectations are the same as for [komorebi](https://github.com/LGUG2Z/komorebi/#development).

Contributors can add new configuration settings by creating a new file in the `registry` directory and implementing
the `registry::Setting` trait, and then updating the `export` and `apply` functions on the `Config` struct in `lib.rs`

For configuration settings that have only a disabled or enabled state, it is possible to
largely copy and paste from existing settings like `transparency_effects` and make the required modifications to point
to the desired registry entry.

Currently, the TOML file format is used to store and read configuration settings from `nazm.toml` files.

- Existing configuration can be exported to the terminal with `nazm export`
- A diff between the existing system configuration and a configuration file can be shown with `nazm diff nazm.toml`
- Changes from a configuration file can be applied with `nazm apply nazm.toml`

It goes without saying that you should create regular system restore points if you would like to use this tool as an
end-user or run it as a contributor.
