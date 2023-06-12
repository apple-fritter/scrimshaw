# Scrimshaw

Scrimshaw is a Rust-based program designed to parse and analyze nested character-separated IRC log files, stored in the [driftwood](https://github.com/apple-fritter/driftwood) format. It allows you to search for specific user messages, extract the corresponding message text then store it in an appropriately labeled text file.

For more information on the driftwood standard, please visit the [driftwood repository](https://github.com/apple-fritter/driftwood).

### A Proof of Concept

Scrimshaw stands as the pioneering proof of concept and scraping implementation of the [driftwood standard](https://github.com/apple-fritter/driftwood). Scrimshaw showcases the practical application of Driftwood; With its unique Unicode character-based parsing approach and nested file structure handling, Scrimshaw demonstrates the feasibility and potential of the Driftwood standard.

By offering efficient log file parsing, targeted message extraction and output generation, Scrimshaw sets the stage for future advancements in IRC log analysis and data extraction, and serves as a foundational project in the Driftwood ecosystem, paving the way for further developments and standardization efforts in the realm of IRC log processing.

## Index

- [Requirements](#requirements)
- [Usage](#usage)
- [Suppressing input](#suppressing-input)
- [Flowchart](#flowchart)
- [Potential Concerns](#potential-concerns)
- [IRC Related Repositories](#irc-meta)
- [IRC usage considerations](#irc-usage-considerations)
- [Disclaimer](#disclaimer)
- [License](#license)

## Requirements

- Rust (stable)
- Cargo
- Logs formatted in the [driftwood standard](https://github.com/apple-fritter/driftwood)
- Consent

## Usage
To use Scrimshaw for parsing and extracting data from IRC log files, follow these steps:

### Installation:

#### Clone the Scrimshaw repository from GitHub:
```bash
git clone https://github.com/apple-fritter/scrimshaw.git
```

#### Navigate to the project directory:
```bash
cd scrimshaw
```

#### Install the required dependencies using Cargo, the Rust package manager:
```bash
cargo build --release
```

### Running Scrimshaw:

Once the dependencies are installed, you can run Scrimshaw using the following command:
```bash
cargo run --release -- [options] <username> <log_path>
```

> Replace `username` with the desired username to search for in the log files and `log_path` with the path to the root directory of the IRC log files.

#### Parsing and Extraction:
Scrimshaw will recursively search for log files in the specified directory structure. It will parse each log entry, looking for the specified username in the sender column. If a log entry contains the username in the appropriate column, it will extract the message text and save it to a file named for the username appended with "_quotes.txt" in the output directory.
    
#### Analyzing the Results:
Navigate to the output directory to find the generated quote files. Open the `<username>_quotes.txt` file to view the extracted message texts. The file will only contain the message texts, with one message per line.
    
### Example Usage:
To search for all messages by the username "GitHubFAN23" in the IRC log files located in the directory structure described in the organizational method section, run the following command:
```bash
cargo run --release -- GitHubFAN23 /path/to/log/files
```

The extracted message texts will be saved in the file `GitHubFAN23_quotes.txt`.

Feel free to customize and integrate Scrimshaw into your own Rust projects or workflows to efficiently parse and extract data from IRC log files.

> Please ensure that you have the necessary permissions to access and process the IRC log files, and respect any data privacy or usage policies in place.
    
---

## Suppressing input
The '#' symbol in Column 1 of the IRC log files serves as a powerful tool to suppress specific lines of data from the parsing process in Scrimshaw. This feature allows users to selectively exclude certain content for any purpose they deem necessary.

The use of '#' as a suppression marker is flexible and versatile, accommodating a wide range of scenarios. It enables users to hide sensitive information, offensive or triggering content, illegal data, or any other data that is irrelevant to the parsing process. By simply prefixing a line with '#', users can ensure that it is ignored during the data extraction phase.

This functionality offers a level of control and discretion, empowering users to filter out content they do not wish to include in the final output. It allows for greater data privacy, content moderation, and adherence to legal or ethical considerations when working with IRC log files.

When using Scrimshaw, it is important to respect data privacy, adhere to applicable regulations, and consider the implications of suppressing data. Exercise caution when dealing with sensitive information and ensure compliance with any relevant policies or guidelines.

By leveraging the '#' symbol in Column 1, Scrimshaw provides a convenient mechanism to exclude specific lines from the parsing process, enabling users to have greater control over the content they extract and process.

---

## Flowchart
```
Start Program
|
├─ Read Command-line Arguments
│   ├─ Extract Username and Log Directory
│   ├─ Validate Arguments
│   └─ Display Usage Instructions if Invalid Arguments
|
├─ Traverse Log Directory
│   ├─ Find Log Files Recursively
│   │   ├─ Check if Entry is File
│   │   │   ├─ Check File Extension
│   │   │   │   ├─ Collect Log Files
│   │   │   │   └─ (Recursive Call)
│   │   │   └─ (Recursive Call)
│   │   └─ (Recursive Call)
│   └─ Sort Log Files by Date (Optional)
|
├─ Parse Log Files
│   ├─ Read File Content
│   ├─ Iterate Over Lines
│   │   ├─ Ignore Lines Starting with '#'
│   │   ├─ Split Line by Separator
│   │   ├─ Check User Match
│   │   │   └─ Collect Message Text
│   │   └─ (Iterate Next Line)
│   └─ (Iterate Next File)
|
└─ Output Message Data
    ├─ Create Output File
    ├─ Write Message Text to File
    └─ End Program
```
## Potential Concerns
#### Input Validation:
Ensure proper validation and error handling for user input, such as the username and log directory. Validate that the log directory exists and contains the expected log files.

#### File Size and Memory:
If the log files are extremely large or numerous, reading and parsing them all at once could consume a significant amount of memory. Consider implementing streaming or chunked processing if memory usage is a concern.

#### Performance:
Depending on the size and complexity of the log files, parsing and searching through them can be a computationally intensive task. Optimize the code for performance by using efficient algorithms and data structures.

#### Security and Privacy:
If the log files contain sensitive information, ensure proper security measures are in place, such as access control and encryption, to protect the data.

#### Testing:
Thoroughly test the program with different input scenarios and edge cases to uncover and address any potential bugs or issues.

---

## IRC Meta

### WeeChat
- [weechat.ban-evasion-detection](https://github.com/apple-fritter/weechat.ban-evasion-detection): Detect and prevent ban evasion. (Python)
- [weechat.typo-aggregator](https://github.com/apple-fritter/weechat.typo-aggregator): Record misspelled words in a TSV (tab-separated values) file. (Python)
- [weechat.whois-aggregator](https://github.com/apple-fritter/weechat.whois-aggregator): Aggregate whois data in a rolling CSV file. (Python)
- [weechat.youtube-info](https://github.com/apple-fritter/weechat.youtube-info): Deprecated. Extract video information from a YouTube URL and post it back to the channel. (Python)
- [weechat.youtube-api](https://github.com/apple-fritter/weechat.youtube-api): Extract video information from a YouTube URL and post it back to the channel. (Python)

### IRCcloud
- [irccloud-to-weechat](https://github.com/apple-fritter/irccloud-to-weechat): Convert IRC logs from IRCcloud format to Weechat format. (Rust)
- [irccloud-to-xchat](https://github.com/apple-fritter/irccloud-to-xchat): Convert IRC logs from IRCcloud format to XChat format. (Rust)

### X-Chat
- [xchat.channel-moderation](https://github.com/apple-fritter/xchat.channel-moderation): Moderate an IRC channel. (Python)
- [doppelganger](https://github.com/apple-fritter/doppelganger): X-Chat mIRC imposter. Fingerprint subversion. (Python bundle)

### Other
- [driftwood](https://github.com/apple-fritter/driftwood): A unified IRC log format definition. (Rust)
- [scrimshaw](https://github.com/apple-fritter/scrimshaw): Create a quoteslist of any given user, from your driftwood formatted logs. (Rust)

### IRC usage considerations
When working with any project involving IRC (Internet Relay Chat), it's important to keep the following considerations in mind to ensure a positive and respectful environment for all participants.

#### Philosophy of Use
Tailor your project's behavior and responses to align with the expected norms and conventions of IRC. Take into account the preferences and expectations of IRC users, ensuring that your project provides a seamless and familiar experience within the IRC ecosystem.

#### Foster a Positive and Inclusive Environment
Respect and adhere to the guidelines and policies of the IRC platform you are using. Familiarize yourself with the platform's rules regarding script usage, automation, and acceptable behavior. Comply with the platform's Terms of Service, and be mindful of any limitations or restrictions imposed by the platform. Strive to create an inclusive and welcoming environment where all users can engage respectfully and comfortably.

#### Respect the Rights and Dignity of Other Users
Maintain a polite and courteous demeanor in all interactions. Uphold the fundamental principles of respect, avoiding engagement in illegal, inappropriate, or offensive behavior. This includes refraining from using derogatory or inflammatory language, sharing explicit, triggering, or offensive content, engaging in harassment, or launching personal attacks. Obtain explicit consent before interacting with other users or sending automated responses. Respect the privacy of other users and avoid invading their personal space without their permission.

#### Respect the IRC Community and Channels
Avoid disrupting the normal flow of conversation within IRC channels. Ensure that your project's actions and responses do not cause unnecessary disruptions or inconvenience to other users. Implement mechanisms to prevent spamming or flooding the channel with excessive or irrelevant messages. Handle errors gracefully, preventing unintended behavior or disruptions to the IRC platform or the experiences of other users.

#### Ensure Compatibility
Consider the potential variations in behavior across different IRC platforms and clients. While aiming for compatibility, be aware that certain functionalities may not be available or consistent across all platforms. Test your project on multiple IRC platforms and clients to ensure compatibility and provide the best possible experience for users.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License

These files released under the [MIT License](LICENSE).
