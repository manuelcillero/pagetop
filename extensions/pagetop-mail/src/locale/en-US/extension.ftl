extension_name = Mail
extension_description = Sends emails over SMTP, sendmail or files, with templates, HTML and attachments.

mail_footer = Message sent by { $app }.

queue_job_title = Send queued emails

# **< Email previews >**

preview_title       = Email previews
preview_empty       = No extension declares email previews.
preview_back        = Back to the list
preview_from        = From
preview_reply_to    = Reply to
preview_to          = To
preview_cc          = Cc
preview_bcc         = Bcc
preview_subject     = Subject
preview_attachments = Attachments
preview_html        = HTML
preview_text        = Plain text
preview_none        = None

# **< Test email >**

perm_send_test         = Send test emails from the administration panel
test_title             = Email
test_description       = Delivery settings and test email.
test_settings          = Current settings
test_transport         = Transport
test_from              = Sender
test_reply_to          = Reply to
test_redirect_to       = Every email is delivered to
test_smtp_server       = SMTP server
test_smtp_security     = Security
test_sendmail_command  = Program
test_file_dir          = Directory
test_not_set           = Not set
test_to                = Send a test email to
test_send              = Send test email
test_subject           = Test email from { $app }
test_body              = This email confirms that { $app } can send messages with the current settings.
test_sent              = Test email sent to { $to }.
test_failed            = The test email could not be sent: { $error }
test_invalid_address   = "{ $to }" is not a valid email address.
test_log_warning       = With the current transport emails are not sent: they are written to the logs.
test_memory_warning    = With the current transport emails are not sent: they are kept in memory.
transport_smtp         = SMTP
transport_sendmail     = sendmail program
transport_file         = .eml files
transport_log          = Logs (not sent)
transport_memory       = Memory (not sent)
security_none          = Unencrypted
security_opportunistic = STARTTLS if the server supports it
security_starttls      = STARTTLS required
security_tls           = TLS from the connection
