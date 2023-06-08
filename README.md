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

### Security and Privacy:
If the log files contain sensitive information, ensure proper security measures are in place, such as access control and encryption, to protect the data.

#### Testing:
Thoroughly test the program with different input scenarios and edge cases to uncover and address any potential bugs or issues.

---

## [Disclaimer](DISCLAIMER)
**This software is provided "as is" and without warranty of any kind**, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

**The authors do not endorse or support any harmful or malicious activities** that may be carried out with the software. It is the user's responsibility to ensure that their use of the software complies with all applicable laws and regulations.

---

## License

These files released under the [MIT License](LICENSE).
