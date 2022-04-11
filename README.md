# rusted-away
Rust CLI to work with r/place [official dataset](https://www.reddit.com/r/place/comments/txvk2d/rplace_datasets_april_fools_2022/) and [unofficial raw dataset](https://archive.org/details/place2022-opl-raw)

## Motivation  
During the r/place there were a lot of communities going together to make artwork that represents their community.
After it ended there was no simple way to figure out who to reward. I later found the unofficial raw dataset that wasn't processed.
I started messing around with it, but realized there was unprecise pixel position as it lacked canvas ids. 

A few days later the official dataset was released, and gave me the opportunity to use both of them together to fix the unprecise pixel position.
I messed around and had a working solution but it wasn't user friendly, so I decided to start rusted-away to make it more friendly.

## Installation (Development)

1. Follow [Rust installation guide](https://www.rust-lang.org/tools/install) to get a rust development environment ready.
2. Clone the repository.
3. Run cargo build to get binaries

## License
[MIT](https://choosealicense.com/licenses/mit/)
