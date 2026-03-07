# Apple Mail

Apple Mail.app integration via AppleScript. Works reliably without special permissions on macOS.

Single Rust binary. Read inbox, search emails, send emails, reply, manage messages, download attachments (including inline MIME parts), and save emails.

## Build

```bash
cargo build --release
```

The binary is at `./target/release/apple-mail`.

## Commands

```
apple-mail <command> [options]
```

| Command | Usage | Description |
|---------|-------|-------------|
| `launch` | `launch` | Launch Mail.app |
| `is-running` | `is-running` | Check if Mail.app is running |
| `unread-count` | `unread-count` | Get total unread messages |
| `unread-count-period` | `unread-count-period --hours 24` | Unread in time period |
| `messages-period` | `messages-period --hours 48` | List messages from time period |
| `mailboxes` | `mailboxes` | List all mailboxes |
| `accounts` | `accounts` | List all accounts |
| `list` | `list [mailbox] [-a account] [-l limit]` | List recent messages |
| `search` | `search "query"` | Search messages by subject/sender |
| `read` | `read <message-id>` | Read specific message |
| `compose` | `compose --to "email" --subject "subj" --body "text"` | Send new email |
| `reply` | `reply <message-id> --body "text" [--reply-all]` | Reply to message |
| `mark-read` | `mark-read <id1> <id2> ...` | Mark as read (batch) |
| `mark-unread` | `mark-unread <id1> ...` | Mark as unread |
| `delete` | `delete <id1> <id2> ...` | Delete messages (batch) |
| `refresh` | `refresh [account]` | Refresh mailboxes |
| `download-attachments` | `download-attachments <message-id> <directory>` | Download attachments via AppleScript |
| `download-all-parts` | `download-all-parts <message-id> <directory>` | Extract all MIME parts (inline + attached) |
| `save-email` | `save-email <message-id> <file-path>` | Save email to file |
| `selection` | `selection` | Get currently selected message |

## Examples

```bash
# List 10 recent inbox messages
apple-mail list -l 10

# List from specific account/mailbox
apple-mail list INBOX -a "Gmail" -l 5

# Search
apple-mail search "meeting"

# Read a message
apple-mail read 12345

# Compose
apple-mail compose --to "user@example.com" --subject "Hi" --body "Hello"

# Reply
apple-mail reply 12345 --body "Thanks!"

# Batch mark as read
apple-mail mark-read 12345 12346 12347

# Download attachments (AppleScript-based)
apple-mail download-attachments 12345 ~/Downloads/

# Download all MIME parts (inline images, signatures, etc.)
apple-mail download-all-parts 12345 ~/Downloads/

# Save email to file
apple-mail save-email 12345 ~/Documents/email.txt

# Time period queries
apple-mail unread-count-period --hours 24
apple-mail messages-period --days 7 -l 50

# Refresh
apple-mail refresh
apple-mail refresh "Gmail"
```

## Output Format

List/search returns: `ID | ReadStatus | Date | Sender | Subject | Att:N`

- `●` = unread, blank = read
- `Att:N` = number of attachments

## Attachment Downloads

Two commands for downloading attachments:

- **`download-attachments`** — Uses AppleScript's `mail attachments`. Fast, works for most cases.
- **`download-all-parts`** — Parses raw MIME source. Finds everything including inline images and S/MIME signatures that AppleScript may miss. Output includes disposition type, content-type, and size.

`download-all-parts` always finds equal or more parts than `download-attachments`.

## Technical Details

- All operations use AppleScript — no direct database access required
- Works without Full Disk Access or special permissions
- Reliable on modern macOS (14.0+)
- Single Rust binary, no external dependencies at runtime
- MIME parsing via `mailparse` crate for `download-all-parts`

## Project Structure

```
src/
├── main.rs          # CLI interface (clap)
├── lib.rs           # Library exports
├── applescript.rs   # AppleScript execution and all mail operations
└── error.rs         # Error types
```
