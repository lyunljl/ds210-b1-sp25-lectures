# Setup

## MOC

The ndarray-linalg crate should be configured correctly for the Rust VSCode images on Mass. Open Cloud.

## MacOS

🧩 Backend Options and Compatibility

ndarray-linalg relies on external BLAS/LAPACK libraries for its linear algebra operations. 

On macOS, the following backends are commonly used: ￼
* OpenBLAS:
* System Installation: You can install OpenBLAS via Homebrew and use the openblas-system feature in your Cargo.toml. This approach has been reported to work on Apple Silicon when OpenBLAS is properly installed.
* Static Linking: Using the openblas-static feature to build OpenBLAS from source can lead to compilation issues on macOS, particularly on M1/M2 chips. Users have reported linker errors and missing symbols when attempting this approach.  ￼ ￼
* Intel MKL:
* The intel-mkl backend is supported on macOS and can be used with ndarray-linalg. However, it requires accepting Intel’s license terms.  ￼
* Accelerate Framework:
* Apple’s Accelerate framework is a native BLAS/LAPACK implementation on macOS. While ndarray can utilize Accelerate via the blas-src crate with the accelerate feature, ndarray-linalg does not currently support Accelerate as a backend. This limitation means that you cannot use ndarray-linalg with Accelerate directly.  ￼ ￼

⚙️ Recommended Setup for macOS

To use ndarray-linalg on macOS, especially on Apple Silicon, consider the following setup:

1.	Install OpenBLAS:
    * Use Homebrew to install OpenBLAS:

```sh
brew install openblas
```

2.	Configure Cargo.toml:
    * Specify the openblas-system feature for ndarray-linalg:

```sh
[dependencies]
ndarray = "0.15"
ndarray-linalg = { version = "0.16", features = ["openblas-system"] }
```

3.	Set Environment Variables:
    * Ensure that the compiler can find OpenBLAS by setting the appropriate environment variables:

```sh
export OPENBLAS_DIR=$(brew --prefix openblas)
export OPENBLAS_INCLUDE_DIR=$OPENBLAS_DIR/include
export OPENBLAS_LIB_DIR=$OPENBLAS_DIR/lib
```

4.	Build Your Project:
    * Compile your project as usual:

```sh
cargo build
```


This configuration has been reported to work on macOS systems with OpenBLAS installed via Homebrew.  ￼ ￼ ￼

🛠 Troubleshooting Tips

* Linker Errors:
    * If you encounter linker errors related to missing symbols (e.g., cblas_* functions), ensure that the correct backend is specified and that the necessary environment variables point to the installed BLAS library. ￼
* Accelerate Framework:
    * As ndarray-linalg does not support the Accelerate framework, attempting to use it as a backend will result in build failures. Stick to supported backends like OpenBLAS or Intel MKL. ￼ ￼
* Apple Silicon Considerations:
    * On M1/M2 Macs, prefer using the openblas-system feature with a Homebrew-installed OpenBLAS. Avoid the openblas-static feature, as it may lead to build issues on these architectures. ￼ ￼

