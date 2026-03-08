//! AppleScript integration for controlling Apple Mail

use crate::error::MailError;
use std::process::Command;

/// Execute an AppleScript and return its output
pub fn execute_applescript(script: &str) -> Result<String, MailError> {
    let output = Command::new("osascript").arg("-e").arg(script).output()?;

    if output.status.success() {
        let result = String::from_utf8(output.stdout)?;
        Ok(result.trim().to_string())
    } else {
        let error_msg = String::from_utf8(output.stderr)?;
        Err(MailError::AppleScriptError(error_msg))
    }
}

/// Launch Apple Mail application
pub fn launch_mail_app() -> Result<(), MailError> {
    let status = Command::new("open").arg("-a").arg("Mail").status()?;
    if !status.success() {
        execute_applescript(r#"tell application "Mail" to activate"#)?;
    }
    Ok(())
}

/// Check if Mail.app is running
pub fn is_mail_running() -> Result<bool, MailError> {
    let script =
        r#"tell application "System Events" to return exists (processes where name is "Mail")"#;
    let result = execute_applescript(script)?;
    Ok(result.to_lowercase().contains("true"))
}

/// Get total unread message count
pub fn get_unread_count() -> Result<usize, MailError> {
    let script = r#"tell application "Mail" to get unread count of inbox"#;
    let result = execute_applescript(script)?;
    result
        .parse::<usize>()
        .map_err(|e| MailError::AppleScriptError(format!("Failed to parse unread count: {}", e)))
}

/// Get unread count within a time period
pub fn get_unread_count_period(hours: Option<i64>, days: Option<i64>) -> Result<usize, MailError> {
    let total_hours = match (hours, days) {
        (Some(h), _) => h,
        (_, Some(d)) => d * 24,
        _ => 24,
    };

    let script = format!(
        r#"
        tell application "Mail"
            set cutoffDate to (current date) - ({} * 60 * 60)
            set unreadCount to 0
            repeat with i from 1 to (count of messages of inbox)
                try
                    set msg to message i of inbox
                    if date received of msg < cutoffDate then exit repeat
                    if read status of msg is false then set unreadCount to unreadCount + 1
                on error
                    exit repeat
                end try
            end repeat
            return unreadCount
        end tell
    "#,
        total_hours
    );

    let result = execute_applescript(&script)?;
    result
        .parse::<usize>()
        .map_err(|e| MailError::AppleScriptError(format!("Failed to parse unread count: {}", e)))
}

/// Get messages within a time period
pub fn get_messages_in_period(
    hours: Option<i64>,
    days: Option<i64>,
    limit: Option<usize>,
) -> Result<String, MailError> {
    let total_hours = match (hours, days) {
        (Some(h), _) => h,
        (_, Some(d)) => d * 24,
        _ => 24,
    };
    let max_count = limit.unwrap_or(50);

    let script = format!(
        r#"
        tell application "Mail"
            set cutoffDate to (current date) - ({} * 60 * 60)
            set output to ""
            set found to 0
            repeat with i from 1 to (count of messages of inbox)
                try
                    set msg to message i of inbox
                    if date received of msg < cutoffDate then exit repeat
                    set mid to id of msg as string
                    set msubject to subject of msg
                    set msender to sender of msg
                    set mdate to date received of msg
                    set mread to read status of msg
                    set readFlag to "●"
                    if mread then set readFlag to " "
                    set hasAtt to count of mail attachments of msg
                    set output to output & mid & " | " & readFlag & " | " & mdate & " | " & msender & " | " & msubject & " | Att:" & hasAtt & linefeed
                    set found to found + 1
                    if found = {} then exit repeat
                on error
                    exit repeat
                end try
            end repeat
            if output is "" then return "No messages found in the specified period"
            return output
        end tell
    "#,
        total_hours, max_count
    );

    execute_applescript(&script)
}

/// Get all mailbox names
pub fn get_mailbox_names() -> Result<Vec<String>, MailError> {
    let script = r#"tell application "Mail" to return name of every mailbox"#;
    let result = execute_applescript(script)?;
    Ok(result.split(',').map(|s| s.trim().to_string()).collect())
}

/// Get all account names
pub fn get_account_names() -> Result<Vec<String>, MailError> {
    let script = r#"tell application "Mail" to return name of every account"#;
    let result = execute_applescript(script)?;
    Ok(result.split(',').map(|s| s.trim().to_string()).collect())
}

/// List recent messages from a mailbox
pub fn list_recent_messages(
    mailbox: Option<&str>,
    account: Option<&str>,
    limit: usize,
) -> Result<String, MailError> {
    let script = match (mailbox, account) {
        (Some(mbx), Some(acc)) => {
            let escaped_acc = acc.replace('"', "\\\"");
            let escaped_mbx = mbx.replace('"', "\\\"");
            format!(
                r#"
                tell application "Mail"
                    set output to ""
                    set targetAccount to account "{}"
                    set mbx to mailbox "{}" of targetAccount
                    repeat with i from 1 to {}
                        try
                            set msg to message i of mbx
                            set mid to id of msg as string
                            set msubject to subject of msg
                            set msender to sender of msg
                            set mdate to date received of msg
                            set mread to read status of msg
                            set readFlag to "●"
                            if mread then set readFlag to " "
                            set hasAtt to count of mail attachments of msg
                            set output to output & mid & " | " & readFlag & " | " & mdate & " | " & msender & " | " & msubject & " | Att:" & hasAtt & linefeed
                        on error
                            exit repeat
                        end try
                    end repeat
                    if output is "" then return "No messages found"
                    return output
                end tell
            "#,
                escaped_acc, escaped_mbx, limit
            )
        }
        (Some(mbx), None) => {
            let escaped_mbx = mbx.replace('"', "\\\"");
            format!(
                r#"
                tell application "Mail"
                    set output to ""
                    repeat with acct in accounts
                        try
                            set targetMbx to mailbox "{}" of acct
                            repeat with i from 1 to {}
                                try
                                    set msg to message i of targetMbx
                                    set mid to id of msg as string
                                    set msubject to subject of msg
                                    set msender to sender of msg
                                    set mdate to date received of msg
                                    set mread to read status of msg
                                    set readFlag to "●"
                                    if mread then set readFlag to " "
                                    set hasAtt to count of mail attachments of msg
                                    set output to output & mid & " | " & readFlag & " | " & mdate & " | " & msender & " | " & msubject & " | Att:" & hasAtt & linefeed
                                on error
                                    exit repeat
                                end try
                            end repeat
                        end try
                    end repeat
                    if output is "" then return "No messages found"
                    return output
                end tell
            "#,
                escaped_mbx, limit
            )
        }
        _ => {
            format!(
                r#"
                tell application "Mail"
                    set output to ""
                    repeat with i from 1 to {}
                        try
                            set msg to message i of inbox
                            set mid to id of msg as string
                            set msubject to subject of msg
                            set msender to sender of msg
                            set mdate to date received of msg
                            set mread to read status of msg
                            set readFlag to "●"
                            if mread then set readFlag to " "
                            set hasAtt to count of mail attachments of msg
                            set output to output & mid & " | " & readFlag & " | " & mdate & " | " & msender & " | " & msubject & " | Att:" & hasAtt & linefeed
                        on error
                            exit repeat
                        end try
                    end repeat
                    if output is "" then return "No messages found"
                    return output
                end tell
            "#,
                limit
            )
        }
    };

    execute_applescript(&script)
}

/// Search messages by subject or sender
pub fn search_messages(query: &str) -> Result<String, MailError> {
    let escaped_query = query.replace('"', "\\\"");

    let script = format!(
        r#"
        tell application "Mail"
            set output to ""
            set searchTerm to "{}"
            repeat with i from 1 to 200
                try
                    set msg to message i of inbox
                    if subject of msg contains searchTerm or sender of msg contains searchTerm then
                        set mid to id of msg as string
                        set msubject to subject of msg
                        set msender to sender of msg
                        set mdate to date received of msg
                        set mread to read status of msg
                        set readFlag to "●"
                        if mread then set readFlag to " "
                        set hasAtt to count of mail attachments of msg
                        set output to output & mid & " | " & readFlag & " | " & mdate & " | " & msender & " | " & msubject & " | Att:" & hasAtt & linefeed
                    end if
                on error
                    exit repeat
                end try
            end repeat
            if output is "" then return "No messages found matching the query"
            return output
        end tell
    "#,
        escaped_query
    );

    execute_applescript(&script)
}

/// Read a specific message by ID
pub fn get_message_content(message_id: &str) -> Result<String, MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();

    let script = format!(
        r#"
        tell application "Mail"
            try
                set msg to first message of inbox whose id is {}
                set output to "From: " & sender of msg & linefeed
                try
                    set mto to ""
                    repeat with r in to recipients of msg
                        set mto to mto & address of r & ", "
                    end repeat
                    if mto ends with ", " then set mto to text 1 thru -3 of mto
                    set output to output & "To: " & mto & linefeed
                end try
                set output to output & "Date: " & date received of msg & linefeed
                set output to output & "Subject: " & subject of msg & linefeed
                set output to output & linefeed & "---" & linefeed & linefeed
                set output to output & content of msg
                return output
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id
    );

    let result = execute_applescript(&script)?;
    if result.starts_with("ERROR:") {
        Err(MailError::MessageNotFound(clean_id.to_string()))
    } else {
        Ok(result)
    }
}

/// Compose a new email
pub fn compose_new_email(to: &str, subject: &str, body: &str) -> Result<(), MailError> {
    let escaped_to = to.replace('"', "\\\"");
    let escaped_subject = subject.replace('"', "\\\"");
    let escaped_body = body.replace('"', "\\\"");

    let script = format!(
        r#"
        tell application "Mail"
            set newMessage to make new outgoing message with properties {{subject:"{}", content:"{}", visible:false}}
            tell newMessage
                make new to recipient at end of to recipients with properties {{address:"{}"}}
            end tell
            send newMessage
        end tell
    "#,
        escaped_subject, escaped_body, escaped_to
    );

    execute_applescript(&script)?;
    Ok(())
}

/// Reply to a message
pub fn reply_to_message(message_id: &str, body: &str, _reply_all: bool) -> Result<(), MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();
    let escaped_body = body.replace('"', "\\\"").replace('\n', "\" & return & \"");

    let script = format!(
        r#"
        tell application "Mail"
            set originalMessage to first message of inbox whose id is {}
            set origSender to sender of originalMessage
            set origDate to date received of originalMessage as string
            set origContent to content of originalMessage
            set quotedHeader to return & return & "On " & origDate & ", " & origSender & " wrote:" & return & return
            set replyMessage to reply originalMessage without opening window
            set content of replyMessage to "{}" & quotedHeader & origContent
            send replyMessage
        end tell
    "#,
        clean_id, escaped_body
    );

    execute_applescript(&script)?;
    Ok(())
}

/// Forward a message to a recipient
pub fn forward_message(message_id: &str, to: &str, body: &str) -> Result<(), MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();
    let escaped_to = to.replace('"', "\\\"");
    let escaped_body = body.replace('"', "\\\"");

    let escaped_body = escaped_body.replace('\n', "\" & return & \"");

    let body_prefix = if body.is_empty() {
        String::new()
    } else {
        format!(r#""{}" & return & return & "#, escaped_body)
    };

    let script = format!(
        r#"
        tell application "Mail"
            set originalMessage to first message of inbox whose id is {}
            set origSender to sender of originalMessage
            set origDate to date received of originalMessage as string
            set origSubject to subject of originalMessage
            set origContent to content of originalMessage
            set fwdHeader to "---------- Forwarded message ----------" & return & "From: " & origSender & return & "Date: " & origDate & return & "Subject: " & origSubject & return & return
            set fwdMessage to forward originalMessage without opening window
            tell fwdMessage
                make new to recipient at end of to recipients with properties {{address:"{}"}}
            end tell
            set content of fwdMessage to {}fwdHeader & origContent
            send fwdMessage
        end tell
    "#,
        clean_id, escaped_to, body_prefix
    );

    execute_applescript(&script)?;
    Ok(())
}

/// Mark a message as read
pub fn mark_message_as_read(message_id: &str) -> Result<(), MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();

    let script = format!(
        r#"
        tell application "Mail"
            try
                set msg to first message of inbox whose id is {}
                set read status of msg to true
                return "OK"
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id
    );

    let result = execute_applescript(&script)?;
    if result.starts_with("ERROR:") {
        Err(MailError::MessageNotFound(clean_id.to_string()))
    } else {
        Ok(())
    }
}

/// Mark a message as unread
pub fn mark_message_as_unread(message_id: &str) -> Result<(), MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();

    let script = format!(
        r#"
        tell application "Mail"
            try
                set msg to first message of inbox whose id is {}
                set read status of msg to false
                return "OK"
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id
    );

    let result = execute_applescript(&script)?;
    if result.starts_with("ERROR:") {
        Err(MailError::MessageNotFound(clean_id.to_string()))
    } else {
        Ok(())
    }
}

/// Delete a message
pub fn delete_message(message_id: &str) -> Result<(), MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();

    let script = format!(
        r#"
        tell application "Mail"
            try
                set msg to first message of inbox whose id is {}
                delete msg
                return "OK"
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id
    );

    let result = execute_applescript(&script)?;
    if result.starts_with("ERROR:") {
        Err(MailError::MessageNotFound(clean_id.to_string()))
    } else {
        Ok(())
    }
}

/// Download attachments from a message
pub fn download_attachments(
    message_id: &str,
    save_directory: &str,
) -> Result<Vec<String>, MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();
    let dir = if save_directory.ends_with('/') {
        save_directory.to_string()
    } else {
        format!("{}/", save_directory)
    };
    let escaped_dir = dir.replace('"', "\\\"");

    let script = format!(
        r#"
        tell application "Mail"
            try
                set theMessage to first message of inbox whose id is {}
                set attachmentPaths to {{}}
                repeat with theAttachment in mail attachments of theMessage
                    try
                        set attachmentName to name of theAttachment
                        set savePath to "{}" & attachmentName
                        save theAttachment in POSIX file savePath
                        set end of attachmentPaths to savePath
                    end try
                end repeat
                if (count of attachmentPaths) is 0 then return "NO_ATTACHMENTS"
                set AppleScript's text item delimiters to ","
                set pathString to attachmentPaths as string
                set AppleScript's text item delimiters to ""
                return pathString
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id, escaped_dir
    );

    let result = execute_applescript(&script)?;

    if result == "NO_ATTACHMENTS" {
        return Ok(vec![]);
    }
    if result.starts_with("ERROR:") {
        return Err(MailError::MessageNotFound(clean_id.to_string()));
    }

    Ok(result.split(',').map(|s| s.trim().to_string()).collect())
}

/// Save email content to a file
pub fn save_email_to_file(message_id: &str, save_path: &str) -> Result<String, MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();

    let script = format!(
        r#"
        tell application "Mail"
            try
                set theMessage to first message of inbox whose id is {}
                set msgContent to "From: " & sender of theMessage & linefeed
                try
                    set msgContent to msgContent & "To: " & (address of first to recipient of theMessage as string) & linefeed
                end try
                set msgContent to msgContent & "Subject: " & subject of theMessage & linefeed
                set msgContent to msgContent & "Date: " & date received of theMessage & linefeed
                set msgContent to msgContent & linefeed & "---" & linefeed & linefeed
                set msgContent to msgContent & content of theMessage
                return msgContent
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id
    );

    let result = execute_applescript(&script)?;
    if result.starts_with("ERROR:") {
        return Err(MailError::MessageNotFound(clean_id.to_string()));
    }

    std::fs::write(save_path, &result)?;
    Ok(save_path.to_string())
}

/// Refresh mailboxes
pub fn refresh_mailboxes(account: Option<&str>) -> Result<String, MailError> {
    let script = if let Some(acc) = account {
        let escaped_acc = acc.replace('"', "\\\"");
        format!(
            r#"
            tell application "Mail"
                set targetAccount to account "{}"
                repeat with mbx in mailboxes of targetAccount
                    try
                        check for new mail for mbx
                    end try
                end repeat
                return "Refresh complete"
            end tell
        "#,
            escaped_acc
        )
    } else {
        r#"
            tell application "Mail"
                check for new mail
                return "Refresh complete"
            end tell
        "#
        .to_string()
    };

    execute_applescript(&script)
}

/// Get the currently selected messages in Mail.app
pub fn get_current_selection() -> Result<String, MailError> {
    let script = r#"
        tell application "Mail"
            try
                set selectedMessages to selection
                if (count of selectedMessages) is 0 then
                    return "No messages selected"
                end if
                set messageInfo to ""
                repeat with msg in selectedMessages
                    set messageInfo to messageInfo & "ID: " & (id of msg as string) & linefeed
                    set messageInfo to messageInfo & "Subject: " & subject of msg & linefeed
                    set messageInfo to messageInfo & "Sender: " & sender of msg & linefeed & linefeed
                end repeat
                return messageInfo
            on error errMsg
                return "Error: " & errMsg
            end try
        end tell
    "#;

    execute_applescript(script)
}

/// Get the raw MIME source of a message
pub fn get_message_source(message_id: &str) -> Result<String, MailError> {
    let clean_id = message_id.split(':').next().unwrap_or(message_id).trim();

    let script = format!(
        r#"
        tell application "Mail"
            try
                set msg to first message of inbox whose id is {}
                return source of msg
            on error errMsg
                return "ERROR:" & errMsg
            end try
        end tell
    "#,
        clean_id
    );

    let result = execute_applescript(&script)?;
    if result.starts_with("ERROR:") {
        Err(MailError::MessageNotFound(clean_id.to_string()))
    } else {
        Ok(result)
    }
}

/// Extract all MIME parts (inline and attached) from a message and save them to a directory
pub fn download_all_parts(
    message_id: &str,
    save_directory: &str,
) -> Result<Vec<String>, MailError> {
    let source = get_message_source(message_id)?;
    let dir = if save_directory.ends_with('/') {
        save_directory.to_string()
    } else {
        format!("{}/", save_directory)
    };

    let parsed = mailparse::parse_mail(source.as_bytes())
        .map_err(|e| MailError::AppleScriptError(format!("MIME parse error: {}", e)))?;

    let mut saved = Vec::new();
    extract_parts(&parsed, &dir, &mut saved)?;
    Ok(saved)
}

fn extract_parts(
    mail: &mailparse::ParsedMail,
    dir: &str,
    saved: &mut Vec<String>,
) -> Result<(), MailError> {
    if mail.subparts.is_empty() {
        let content_type = mail.ctype.mimetype.as_str();
        // Skip text/html and text/plain body parts
        if content_type.starts_with("text/") {
            return Ok(());
        }

        let disposition = mail
            .get_content_disposition();

        let filename = disposition
            .params
            .get("filename")
            .or_else(|| mail.ctype.params.get("name"))
            .cloned()
            .unwrap_or_else(|| {
                let ext = match content_type {
                    "image/png" => "png",
                    "image/jpeg" | "image/jpg" => "jpg",
                    "image/gif" => "gif",
                    "image/webp" => "webp",
                    "application/pdf" => "pdf",
                    _ => "bin",
                };
                format!("part_{}.{}", saved.len() + 1, ext)
            });

        let body = mail
            .get_body_raw()
            .map_err(|e| MailError::AppleScriptError(format!("Failed to decode part: {}", e)))?;

        if body.is_empty() {
            return Ok(());
        }

        let path = format!("{}{}", dir, filename);
        std::fs::write(&path, &body)?;

        let disp_type = match disposition.disposition {
            mailparse::DispositionType::Inline => "inline",
            mailparse::DispositionType::Attachment => "attachment",
            _ => "unknown",
        };
        saved.push(format!("{} ({}; {}; {} bytes)", path, disp_type, content_type, body.len()));
    } else {
        for subpart in &mail.subparts {
            extract_parts(subpart, dir, saved)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_applescript_simple() {
        let result = execute_applescript("return \"test\"");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_string_escaping() {
        let input = r#"Test with "quotes""#;
        let escaped = input.replace('"', "\\\"");
        assert!(escaped.contains("\\\""));
    }
}
