# IEEE Journal Downloader

* A cross-platform terminal program which tries to download every article in a specified journal and merge the documents into one **PDF** file.
* [YouTube Demo](https://youtu.be/5HWfE48WohY)
* Supported download domains:
    * [SciHub (Recommended, fast)](https://sci-hub.se/)
    * [LibGen (Slow)](https://libgen.is/scimag/)
* **💻 Download:**
    * [⊞ Windows (5.21 MB)](https://github.com/FongYoong/ieee_journal_downloader/releases/download/0.1.0/ieee_journal_downloader.exe)

    * [🐧 Linux (10.6 MB)](https://github.com/FongYoong/ieee_journal_downloader/releases/download/0.1.0/ieee_journal_downloader_linux) .
* ⚠ Possible errors:
    * Windows: If Microsoft Defender SmartScreen appears, click **More Info** and then click **Run Anyway**.
    * Linux:     If permission denied, try `chmod +x ieee_journal_downloader_linux`

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

8) Some journals, especially very recent ones, may not be available:
    ![fail](https://i.imgur.com/R3ETv3Y.png)
***
## Background Info

* [angular_main.js](https://github.com/FongYoong/ieee_journal_downloader/blob/master/misc/angular_main.js) contains the main Angular code of the iEEE journal website. There are plenty REST API links littered throughout the code.
* [requests_tracking_data.txt](https://github.com/FongYoong/ieee_journal_downloader/blob/master/misc/requests_tracking_data.txt) is a list of network requests made by the IEEE journal website. The relevant requests are listed in [relevant_requests.txt](https://github.com/FongYoong/ieee_journal_downloader/blob/master/misc/relevant_requests.txt). 
* [sample_toc_api_data.js](https://github.com/FongYoong/ieee_journal_downloader/blob/master/misc/sample_toc_api_data.js) contains a sample response returned by a POST request to [https://ieeexplore.ieee.org/rest/search/pub/8014/issue/8802299/toc](https://ieeexplore.ieee.org/rest/search/pub/8014/issue/8802299/toc)
* As an example, the minimum required POST request headers are:
    ```
    Accept: application/json, text/plain, */*
    Content-Type: application/json
    Host: ieeexplore.ieee.org
    Origin: https://ieeexplore.ieee.org
    Referer: https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=8802299&punumber=8014
    ```
    whereas the request payload is:
    ```json
    {
        "isnumber":"8802299",
        "punumber":"8014",
        "sortType":"vol-only-seq"
    }
    ```

***
## Building from source

1. Clone this repository 👪
    * `git clone https://github.com/FongYoong/ieee_journal_downloader.git`
2. Install the Rust toolchains (Rustc, Rustup, Cargo).
    * [Follow instructions here](https://www.rust-lang.org/tools/install).
3. For Linux systems, install the following:
    `sudo apt-get install pkg-config libssl-dev libx11-xcb-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev`
4. Move into the cloned repository
    * `cd ieee_journal_downloader`
5. Build! 🔨
    * `cargo build --release`
    or
    * `cargo run --release`