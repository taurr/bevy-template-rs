# Contribution guidelines

First off, thank you for considering contributing to {{project-name}}.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/{{github_username}}/{{project-name}}/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### <a name="CHANGELOG"/>Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/{{github_username}}/{{project-name}}/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

This is a project containing templates for use with [`cargo-generate`], thus it might be a little different.

```shell
git clone https://github.com/taurr/embedded-template-rs
```

To try out and expand a contained template use

```shell
cargo generate --path <path_to_embedded-template-rs>
```

For additional options, see the readme for [`cargo-generate`]

### Useful links

* [`cargo-generate`]
* [`Liquid` syntax](https://shopify.github.io/liquid/basics/introduction/)
* [`Rhai` book](https://rhai.rs/book/)

[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
