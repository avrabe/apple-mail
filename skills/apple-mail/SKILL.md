---
name: apple-mail
description: Apple Mail.app integration via AppleScript. Works reliably without special permissions. Read inbox, search emails, send emails, reply, manage messages, download attachments, and save emails.
metadata: {"bot":{"emoji":"📧","os":["darwin"]}}
---

# Apple Mail

**Apple Mail integration using AppleScript. No special permissions required.**

## Quick Start

```bash
cargo build --release
./target/release/apple-mail <command>
```

## Commands

| Command | Usage | Description |
|---------|-------|-------------|
| **Launch** | `launch` | Launch Mail.app |
| **Check Status** | `is-running` | Check if Mail.app is running |
| **Unread Count** | `unread-count` | Get total unread messages |
| **Unread Period** | `unread-count-period --hours 24` | Unread in time period |
| **Messages Period** | `messages-period --hours 48` | List messages from time period |
| **Mailboxes** | `mailboxes` | List all mailboxes |
| **Accounts** | `accounts` | List all accounts |
| **List** | `list [mailbox] [-a account] [-l limit]` | List recent messages |
| **Search** | `search "query"` | Search messages by subject/sender |
| **Read** | `read <message-id>` | Read specific message |
| **Compose** | `compose --to "email" --subject "subj" --body "text"` | Send new email |
| **Reply** | `reply <message-id> --body "text"` | Reply to message |
| **Mark Read** | `mark-read <id1> <id2>` | Mark as read (batch) |
| **Mark Unread** | `mark-unread <id1>` | Mark as unread |
| **Delete** | `delete <id1> <id2>` | Delete messages (batch) |
| **Refresh** | `refresh [account]` | Refresh mailboxes |
| **Download Attachments** | `download-attachments <message-id> <directory>` | Download attachments |
| **Download All Parts** | `download-all-parts <message-id> <directory>` | Extract all MIME parts (inline + attached) |
| **Save Email** | `save-email <message-id> <file-path>` | Save email to file |
| **Selection** | `selection` | Get currently selected message |

## Examples

### List Recent Messages
```bash
./target/release/apple-mail list
./target/release/apple-mail list INBOX -l 10
./target/release/apple-mail list INBOX -a "Gmail" -l 5
```

### Search Messages
```bash
./target/release/apple-mail search "meeting"
./target/release/apple-mail search "john@example.com"
```

### Read a Message
```bash
./target/release/apple-mail read 12345
```

### Compose New Email
```bash
./target/release/apple-mail compose \
    --to "colleague@company.com" \
    --subject "Project Update" \
    --body "Hi, here is the update..."
```

### Reply to Message
```bash
./target/release/apple-mail reply 12345 --body "Thanks!"
```

### Mark Multiple as Read
```bash
./target/release/apple-mail mark-read 12345 12346 12347
```

### Download Attachments
```bash
./target/release/apple-mail download-attachments 12345 ~/Downloads/
```

### Download All MIME Parts (inline + attached)
```bash
./target/release/apple-mail download-all-parts 12345 ~/Downloads/
```

`download-all-parts` parses the raw MIME source and extracts all non-text parts, including inline images and signatures that `download-attachments` may miss. Output includes disposition, content-type, and size for each part.

### Save Email to File
```bash
./target/release/apple-mail save-email 12345 ~/Documents/email.txt
```

### Time Period Queries
```bash
# Unread in last 24 hours
./target/release/apple-mail unread-count-period --hours 24

# Unread in last 7 days
./target/release/apple-mail unread-count-period --days 7

# Messages from last 48 hours
./target/release/apple-mail messages-period --hours 48 -l 50
```

### Refresh Mailboxes
```bash
./target/release/apple-mail refresh
./target/release/apple-mail refresh "Gmail"
```

## Output Format

List/search returns: `ID | ReadStatus | Date | Sender | Subject | Att:N`
- `●` = unread, blank = read
- `Att:N` = number of attachments

Example:
```
12345 | ● | Sat, 7 Mar 2026 10:30:00 | John Smith | Project Update | Att:2
12346 |   | Sat, 7 Mar 2026 09:15:00 | Jane Doe | Meeting Notes | Att:0
```

## Batch Operations

Multiple message IDs can be passed to mark-read, mark-unread, and delete:

```bash
./target/release/apple-mail mark-read 10001 10002 10003
./target/release/apple-mail delete 10001 10002 10003
```

## Gmail Mailboxes

Gmail special folders need `[Gmail]/` prefix:

| Shows as | Use |
|----------|-----|
| `Spam` | `[Gmail]/Spam` |
| `Sent Mail` | `[Gmail]/Sent Mail` |
| `All Mail` | `[Gmail]/All Mail` |
| `Trash` | `[Gmail]/Trash` |

Custom labels work without prefix.

## Time Period Options

Commands that support time periods:

- `--hours <n>` - Look back n hours (e.g., 24, 12, 6)
- `--days <n>` - Look back n days (e.g., 7, 30, 90)

## Technical Details

- All operations use AppleScript - no direct database access required
- Works without Full Disk Access or special permissions
- Reliable on modern macOS (14.0+)
- Single Rust binary, no external dependencies at runtime

## Error Handling

| Error | Cause | Solution |
|-------|-------|----------|
| Mail.app not running | Mail.app needs to be open | Run `launch` first |
| Message not found | Invalid ID | Get fresh IDs from `list` or `search` |
| Permission denied | macOS restriction | AppleScript commands work without extra permissions |

## Performance

| Operation | Speed | Notes |
|-----------|-------|-------|
| `unread-count` | <1s | Very fast |
| `list` | 1-3s | Depends on limit |
| `search` | 2-5s | Searches last 200 messages |
| `read` | ~1s | Direct message access |
| `delete` | ~0.5s | Fast deletion |
| `mark-read` | ~1s | Quick status update |

## Notes

- Message IDs are internal - get fresh ones from `list` or `search`
- All commands work without special permissions
- Supports batch operations for efficiency
- Attachments and email content can be saved to files
- Compatible with modern macOS privacy protections
