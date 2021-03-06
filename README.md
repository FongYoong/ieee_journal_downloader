# IEEE Journal Downloader

* A cross-platform terminal program which tries to download every article in a specified journal and merge the documents into one **PDF** file.
* **May not work for some new journals**
* [â¶ï¸ YouTube Demo](https://youtu.be/5HWfE48WohY)
* Supported download domains:
    * IEEE if document is free
    * [SciHub (Recommended, fast)](https://sci-hub.se/)
    * [LibGen (Slow)](https://libgen.is/scimag/)
* **ð» Download:**
    * [â Windows (5.24 MB)](https://github.com/FongYoong/ieee_journal_downloader/releases/download/0.4.0/ieee_journal_downloader.exe)

    * [ð§ Linux (10.64Â MB)](https://github.com/FongYoong/ieee_journal_downloader/releases/download/0.4.0/ieee_journal_downloader_linux) .
* â  Possible errors:
    * Windows: If Microsoft Defender SmartScreen appears, click **More Info** and then click **Run Anyway**.
    * Linux:     If permission denied, try `chmod +x ieee_journal_downloader_linux`
* ð Possible bugs:
    * The PDF merging process has some bugs and the page order may get mixed up in some cases. Due to time constraints, I may not fix this anytime soon as I'm more interested in the separate documents.

***
## Usage

1) Identify the **IEEE journal link**.
    Sample link: [https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=8802299&punumber=8014](https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=8802299&punumber=8014)

    ![get_link](https://i.imgur.com/MWBQCRX.png)

2) Double click on the program or invoke it from the command line.

3) Specify the journal link manually or from the clipboard:

    ![specify_link](https://i.imgur.com/FxGNVUg.png)

    Alternatively, the link can be specified as an argument when you start the program from the terminal:
    ```bash
    # Linux
    ./ieee_journal_downloader "your_link_here"
    ./ieee_journal_downloader "https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=9340528&punumber=8475037"

    # Windows
    ieee_journal_downloader.exe "your_link_here"
    ieee_journal_downloader.exe "https://ieeexplore.ieee.org/xpl/tocresult.jsp?isnumber=9340528&punumber=8475037"
    ```

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
* [sample_toc_api_data.json](https://github.com/FongYoong/ieee_journal_downloader/blob/master/misc/sample_toc_api_data.json) contains a sample response returned by a POST request to [https://ieeexplore.ieee.org/rest/search/pub/8014/issue/8802299/toc](https://ieeexplore.ieee.org/rest/search/pub/8014/issue/8802299/toc). This response is used to identify the journal's documents.
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
* [sample_metadata_api_data.json](https://github.com/FongYoong/ieee_journal_downloader/blob/master/misc/sample_metadata_api_data.json) contains a sample response returned by a GET request to [https://ieeexplore.ieee.org/rest/publication/home/metadata?issueid=4381235](https://ieeexplore.ieee.org/rest/publication/home/metadata?issueid=4381235). This metadata is fetched if the user-specified URL does not contain a publication number. The headers should be:
    ```
    Accept: application/json, text/plain, */*
    Content-Type: application/json
    Host: ieeexplore.ieee.org
    Origin: https://ieeexplore.ieee.org
    ```

***
## Building from source

1. Clone this repository ðª
    * `git clone https://github.com/FongYoong/ieee_journal_downloader.git`
2. Install the Rust toolchains (Rustc, Rustup, Cargo).
    * [Follow instructions here](https://www.rust-lang.org/tools/install).
3. For Linux systems, install the following:
    `sudo apt-get install pkg-config libssl-dev libx11-xcb-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev`
4. Move into the cloned repository
    * `cd ieee_journal_downloader`
5. Build! ð¨
    * `cargo build --release`
    or
    * `cargo run --release`