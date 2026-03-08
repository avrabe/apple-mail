use clap::{Parser, Subcommand};
use apple_mail::*;

/// Apple Mail integration via AppleScript
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Launch Apple Mail
    Launch,
    /// Check if Mail.app is running
    IsRunning,
    /// Get unread message count
    UnreadCount,
    /// Get unread count for a time period
    UnreadCountPeriod {
        /// Hours to look back
        #[arg(long)]
        hours: Option<i64>,
        /// Days to look back
        #[arg(long)]
        days: Option<i64>,
    },
    /// List messages from a time period
    MessagesPeriod {
        /// Hours to look back
        #[arg(long)]
        hours: Option<i64>,
        /// Days to look back
        #[arg(long)]
        days: Option<i64>,
        /// Limit number of results
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// List all mailboxes
    Mailboxes,
    /// List all accounts
    Accounts,
    /// Search messages by subject or sender
    Search {
        /// Query to search for
        query: String,
    },
    /// Get current selection in Mail.app
    Selection,
    /// Refresh mailboxes
    Refresh {
        /// Account name (refreshes all if omitted)
        account: Option<String>,
    },
    /// List recent messages
    List {
        /// Mailbox name (default: INBOX)
        mailbox: Option<String>,
        /// Account name
        #[arg(short, long)]
        account: Option<String>,
        /// Number of messages to list
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Read a message by ID
    Read {
        /// Message ID
        message_id: String,
    },
    /// Compose a new email
    Compose {
        /// Recipient email address
        #[arg(long)]
        to: String,
        /// Email subject
        #[arg(long)]
        subject: String,
        /// Email body
        #[arg(long)]
        body: String,
    },
    /// Reply to a message
    Reply {
        /// Message ID to reply to
        message_id: String,
        /// Reply body
        #[arg(long)]
        body: String,
        /// Reply to all recipients
        #[arg(long)]
        reply_all: bool,
    },
    /// Forward a message
    Forward {
        /// Message ID to forward
        message_id: String,
        /// Recipient email address
        #[arg(long)]
        to: String,
        /// Optional body to prepend
        #[arg(long, default_value = "")]
        body: String,
    },
    /// Mark messages as read
    MarkRead {
        /// Message IDs
        message_ids: Vec<String>,
    },
    /// Mark messages as unread
    MarkUnread {
        /// Message IDs
        message_ids: Vec<String>,
    },
    /// Delete messages
    Delete {
        /// Message IDs
        message_ids: Vec<String>,
    },
    /// Download attachments from a message
    DownloadAttachments {
        /// Message ID
        message_id: String,
        /// Directory to save attachments
        directory: String,
    },
    /// Download all MIME parts (inline and attached) from a message
    DownloadAllParts {
        /// Message ID
        message_id: String,
        /// Directory to save parts
        directory: String,
    },
    /// Save email content to a file
    SaveEmail {
        /// Message ID
        message_id: String,
        /// File path to save to
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();

    match args.command {
        Commands::Launch => {
            launch_mail_app()?;
            println!("Mail.app launched successfully");
        }
        Commands::IsRunning => {
            let running = is_mail_running()?;
            println!(
                "Mail.app is {}running",
                if running { "" } else { "not " }
            );
        }
        Commands::UnreadCount => {
            let count = get_unread_count()?;
            println!("Unread messages: {}", count);
        }
        Commands::UnreadCountPeriod { hours, days } => {
            let count = get_unread_count_period(hours, days)?;
            let period = if let Some(h) = hours {
                format!("last {} hours", h)
            } else if let Some(d) = days {
                format!("last {} days", d)
            } else {
                "last 24 hours".to_string()
            };
            println!("Unread messages ({}): {}", period, count);
        }
        Commands::MessagesPeriod { hours, days, limit } => {
            let result = get_messages_in_period(hours, days, limit)?;
            println!("{}", result);
        }
        Commands::Mailboxes => {
            let mailboxes = get_mailbox_names()?;
            for m in mailboxes {
                println!("{}", m);
            }
        }
        Commands::Accounts => {
            let accounts = get_account_names()?;
            for a in accounts {
                println!("{}", a);
            }
        }
        Commands::Search { query } => {
            let result = search_messages(&query)?;
            println!("{}", result);
        }
        Commands::Selection => {
            let result = get_current_selection()?;
            println!("{}", result);
        }
        Commands::Refresh { account } => {
            let result = refresh_mailboxes(account.as_deref())?;
            println!("{}", result);
        }
        Commands::List {
            mailbox,
            account,
            limit,
        } => {
            let result =
                list_recent_messages(mailbox.as_deref(), account.as_deref(), limit)?;
            println!("{}", result);
        }
        Commands::Read { message_id } => {
            let result = get_message_content(&message_id)?;
            println!("{}", result);
        }
        Commands::Compose { to, subject, body } => {
            compose_new_email(&to, &subject, &body)?;
            println!("Email composed successfully");
        }
        Commands::Reply {
            message_id,
            body,
            reply_all,
        } => {
            reply_to_message(&message_id, &body, reply_all)?;
            println!("Reply created successfully");
        }
        Commands::Forward {
            message_id,
            to,
            body,
        } => {
            forward_message(&message_id, &to, &body)?;
            println!("Message forwarded successfully");
        }
        Commands::MarkRead { message_ids } => {
            let mut success = 0;
            let mut failed = 0;
            for id in &message_ids {
                if mark_message_as_read(id).is_ok() {
                    success += 1;
                } else {
                    failed += 1;
                }
            }
            println!("Marked {} as read ({} failed)", success, failed);
        }
        Commands::MarkUnread { message_ids } => {
            let mut success = 0;
            let mut failed = 0;
            for id in &message_ids {
                if mark_message_as_unread(id).is_ok() {
                    success += 1;
                } else {
                    failed += 1;
                }
            }
            println!("Marked {} as unread ({} failed)", success, failed);
        }
        Commands::Delete { message_ids } => {
            let mut success = 0;
            let mut failed = 0;
            for id in &message_ids {
                if delete_message(id).is_ok() {
                    success += 1;
                } else {
                    failed += 1;
                }
            }
            println!("Deleted {} messages ({} failed)", success, failed);
        }
        Commands::DownloadAttachments {
            message_id,
            directory,
        } => {
            let paths = download_attachments(&message_id, &directory)?;
            if paths.is_empty() {
                println!("No attachments found");
            } else {
                println!("Downloaded {} attachments:", paths.len());
                for p in paths {
                    println!("  {}", p);
                }
            }
        }
        Commands::DownloadAllParts {
            message_id,
            directory,
        } => {
            let parts = download_all_parts(&message_id, &directory)?;
            if parts.is_empty() {
                println!("No non-text parts found");
            } else {
                println!("Extracted {} parts:", parts.len());
                for p in parts {
                    println!("  {}", p);
                }
            }
        }
        Commands::SaveEmail { message_id, path } => {
            let saved = save_email_to_file(&message_id, &path)?;
            println!("Email saved to: {}", saved);
        }
    }

    Ok(())
}
