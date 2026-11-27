# SM-T385 Alpha 2 build inputs

This device tree accompanies the November 27, 2026 LineageOS 18.1 test build for
SM-T385 (gta2slte). The ROM and recovery are distributed through the compatibility
repository's Releases page. Windows 11 MTP transfers still need testing.

Changes include AOSP MTP, the sensor service and auto rotation, SELinux enforcing
at boot, Qualcomm H.264 playback, zRAM, and the Rust shims and Game Mode policy.

The device tree alone does not include changes in the projects below. Sync a
LineageOS 18.1 source tree with these project bases, then apply the companion
patches before running `lunch lineage_gta2slte-userdebug` and `m bacon`.

| Project path | Base commit | Patch |
| --- | --- | --- |
| `kernel/samsung/msm8937` | `24810576d36a14f88cc7a089129eee6896938d4b` | [kernel.patch](kernel.patch) |
| `hardware/interfaces` | `bd07a4f9a4247afc67b6495231e11e8704737b40` | [hardware-interfaces.patch](hardware-interfaces.patch) |
| `hardware/qcom-caf/msm8996/media` | `0f04f3c6a6b24ad1fab043a817216df408eeba5d` | [qualcomm-media.patch](qualcomm-media.patch) |
| `prebuilts/rust` | `a48fb0348501e6d2ef7380b7c5e66ebe297e0dfd` | [rust-prebuilts.patch](rust-prebuilts.patch) |
| `frameworks/base` | `bcfb000417038c781e95d7c781c852571903a09f` | [frameworks-base.patch](frameworks-base.patch) |
| `vendor/samsung/gta2slte` | `9d665337dd3226a12b8a673e023bd2ecb8e632eb` | [vendor-init.patch](vendor-init.patch) |

Run these commands from the root of the Android source tree for each row, using
its project path and patch filename:

```sh
patch="$PWD/device/samsung/gta2slte/patches/alpha2/kernel.patch"
git -C kernel/samsung/msm8937 apply --check "$patch"
git -C kernel/samsung/msm8937 apply "$patch"
```

The vendor patch modifies one init script and requires the existing vendor tree.
The patches do not contain vendor binaries.
