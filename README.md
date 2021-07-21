# IEEE Journal Downloader

* A cross-platform terminal program which tries to download every article in a specified journal and merge the documents into one **PDF** file.
* Supported download domains:
    * [SciHub (Recommended, fast)](https://sci-hub.se/)
    * [LibGen (Slow)](https://libgen.is/scimag/)
* **💻 Download**
    * [Windows (5 MB)]()
    * [Linux (5 MB)]()

***
## Usage

1) Double click on the file or invoke it from the command line.

2) Identify the **IEEE journal link**.
    Sample link: [https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=8802299&punumber=8014](https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=8802299&punumber=8014)
    ![get_link](https://i.imgur.com/MWBQCRX.png)

3) Specify the journal link manually or from the clipboard:
    ![specify_link](https://i.imgur.com/FxGNVUg.png)

4) Select the download domain:
    ![domain](https://i.imgur.com/KK9N6ly.png)

5) Wait for the documents to be fetched:
    ![fetch](https://i.imgur.com/kHrtfAY.png)

6) After downloading a couple of documents, the program will wait for 60 seconds to avoid any human captchas:
    ![captcha](https://i.imgur.com/AZjamsg.png)

7) The merged PDF will be stored in the folder **pdf_output** relative to the program's location:
    ![done](https://i.imgur.com/FgNdTvn.png)

***
## Building from source

1. Clone this repository 👪
    * `git clone https://github.com/FongYoong/ieee_journal_downloader.git`
2. Install the Rust toolchains (Rustc, Rustup, Cargo).
    * [Follow instructions here](https://www.rust-lang.org/tools/install).
3. Move into the cloned repository
    * `cd ieee_journal_downloader`
4. Build! 🔨
    * `cargo build --release`
    or
    * `cargo run --release`