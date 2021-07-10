# Performing a release

1. Make a release PR, with two commits: one to set the version to the one being released, and another to set the version to a development version number, for development versioning post-release.

    See example PR: https://github.com/coreos/fedora-coreos-pinger/pull/20

2. Once the release PR has been reviewed and all looks good, merge the PR.

3. Go to the `coreos/fedora-coreos-pinger` GitHub repository web interface, then under Code > Releases, click "Draft a new release". For the commit, choose the one at the version being released (e.g. `cargo: release 0.0.4` in PR #20). Fill in the tag version and release title like the following example. Tag version: `v0.0.4`, Release title: `fedora-coreos-pinger v0.0.4`. Write a brief description summarizing the changes, or listing commits since the last release.

4. From the `coreos/fedora-coreos-pinger` GitHub repository checkout in your filesystem, do the following:

    ```
    git fetch
    git checkout <tag version being released>
    cargo package
    cargo publish
    ```
