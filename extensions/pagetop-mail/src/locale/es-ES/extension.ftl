extension_name = Correo electrónico
extension_description = Envía correos por SMTP, sendmail o archivos, con plantillas, HTML y adjuntos.

mail_footer = Mensaje enviado por { $app }.

queue_job_title = Enviar los correos de la cola

# **< Vistas previas de correos >**

preview_title       = Vistas previas de correos
preview_empty       = Ninguna extensión declara vistas previas de correos.
preview_back        = Volver a la lista
preview_from        = Remitente
preview_reply_to    = Responder a
preview_to          = Para
preview_cc          = Cc
preview_bcc         = Cco
preview_subject     = Asunto
preview_attachments = Adjuntos
preview_html        = HTML
preview_text        = Texto plano
preview_none        = Ninguno

# **< Correo de prueba >**

perm_send_test         = Enviar correos de prueba desde el panel de administración
test_title             = Correo electrónico
test_description       = Configuración del envío y correo de prueba.
test_settings          = Configuración actual
test_transport         = Transporte
test_from              = Remitente
test_reply_to          = Responder a
test_redirect_to       = Todos los correos se entregan a
test_smtp_server       = Servidor SMTP
test_smtp_security     = Seguridad
test_sendmail_command  = Programa
test_file_dir          = Directorio
test_not_set           = Sin configurar
test_to                = Enviar un correo de prueba a
test_send              = Enviar correo de prueba
test_subject           = Correo de prueba de { $app }
test_body              = Este correo confirma que { $app } puede enviar mensajes con la configuración actual.
test_sent              = Correo de prueba enviado a { $to }.
test_failed            = No se pudo enviar el correo de prueba: { $error }
test_invalid_address   = «{ $to }» no es una dirección de correo válida.
test_log_warning       = Con el transporte actual los correos no se envían: se escriben en las trazas.
test_memory_warning    = Con el transporte actual los correos no se envían: se guardan en memoria.
transport_smtp         = SMTP
transport_sendmail     = Programa sendmail
transport_file         = Archivos .eml
transport_log          = Trazas (no se envía)
transport_memory       = Memoria (no se envía)
security_none          = Sin cifrar
security_opportunistic = STARTTLS si el servidor lo admite
security_starttls      = STARTTLS obligatorio
security_tls           = TLS desde la conexión
