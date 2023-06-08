# Scrimshaw

Scrimshaw is a Rust-based program designed to parse and analyze nested character-separated IRC log files, stored in the [driftwood](https://github.com/apple-fritter/driftwood) format. It allows you to search for specific user messages, extract the corresponding message text then store it in an appropriately labeled text file.

For more information on the driftwood standard, please visit the [driftwood repository](https://github.com/apple-fritter/driftwood).

## Requirements

- Rust (stable)
- Cargo

## Usage

1. Clone the Scrimshaw repository:

```bash
git clone https://github.com/your-username/scrimshaw.git
```
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