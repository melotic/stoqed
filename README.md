# stoqed
Notifies you when a class at Virginia Tech has a seat available. Created because CoursePickle is too slow.

Tested on Arch Linux. MacOS should work, as well as Windows, however I have not tested on these platforms.

# Build Instructions

* Install rust ([guide here](https://www.rust-lang.org/tools/install))
* `git clone https://github.com/melotic/stoqed.git`
* `cd stoqed`
* `cargo build --release`

stqoqed will be available in `stoqed/target/release/stoqed`.

# Usage
`stoqed <session id cookie> <term> [CRNs]`

You can obtain your `SESSID` cookie by going into your developer tools while in HokieSPA or the Timetable and viewing your cookies.

The term is the year the semester starts (YYYY) followed by the month the semester starts (MM). For example, the Spring 2021 semester would have a term of `202101`.

stoqed will notify you with a desktop notification when your class has a seat available. stoqed will check the timetable every 15 seconds.

## Examples
### Wait for Music Appreciation, and Software Reverse Engineering:
`stoqed AAAAAAAAAAAAA= 202101 18217 18218 18218 21563`

# Contributing
Pull requests are welcome! :)
