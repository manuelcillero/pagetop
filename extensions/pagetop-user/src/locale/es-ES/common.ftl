## pagetop-user — Español

# **< Metadatos de la extensión >**

extension_name        = PageTop Usuario
extension_description = Identidad de usuario, autenticación, roles y permisos para PageTop.

# **< Títulos de página >**

title-login            = Iniciar sesión
title-register         = Crear cuenta
title-password-reset   = Recuperar contraseña
title-new-password     = Establecer nueva contraseña
title-verify-email     = Confirmar tu correo
title-profile          = Mi perfil
title-profile-edit     = Editar mi perfil
title-profile-password = Cambiar mi contraseña

# **< Etiquetas de campos >**

field-username         = Usuario o email
field-password         = Contraseña
field-email            = Dirección de email
field-confirm-password = Confirmar contraseña
field-new-password     = Nueva contraseña
field-current-password = Contraseña actual
field-remember-me      = Recuérdame

# **< Botones y enlaces >**

btn-login           = Entrar
btn-logout          = Cerrar sesión
btn-register        = Crear cuenta
btn-send-reset-link = Enviar enlace
btn-set-password    = Cambiar contraseña
btn-verify-email    = Confirmar mi correo
btn-edit-profile    = Editar perfil

link-register        = Crear una cuenta
link-forgot-password = ¿Olvidaste tu contraseña?
link-back-to-login   = Volver al inicio de sesión

# **< Menú de cuenta >**

menu-account = Cuenta
menu-admin   = Administración
menu-login   = Iniciar sesión
menu-profile = Perfil

# **< Mensajes >**

msg-password-reset-sent =
    Si existe una cuenta con ese email, hemos enviado un enlace de recuperación.
    Revisa tu bandeja de entrada.

msg-verify-email = Pulsa el botón para confirmar tu dirección de correo y activar tu cuenta.

msg-verification-sent =
    Te hemos enviado un correo para confirmar tu dirección.
    Abre el enlace que contiene y confirma tu dirección para activar tu cuenta.

# **< Correos >**

mail-greeting = Hola, { $username }:
mail-greeting-plain = Hola:
mail-link-expires =
    { $hours ->
        [one] El enlace caduca en una hora.
       *[other] El enlace caduca en { $hours } horas.
    }

mail-reset-subject = Recupera tu contraseña de { $app }
mail-reset-intro   = Hemos recibido una solicitud para restablecer la contraseña de tu cuenta. Para elegir una nueva, abre este enlace:
mail-reset-action  = Elegir una nueva contraseña
mail-reset-ignore  = Si no lo has solicitado tú, ignora este correo: tu contraseña no cambiará.

mail-verify-subject = Confirma tu correo en { $app }
mail-verify-intro   = Gracias por registrarte. Para activar tu cuenta, confirma tu dirección de correo abriendo este enlace:
mail-verify-action  = Confirmar mi correo
mail-verify-ignore  = Si no has creado ninguna cuenta, ignora este correo.

mail-preview-reset  = Recuperación de contraseña
mail-preview-verify = Verificación del correo al registrarse

# **< Mensajes de error >**

error-invalid-credentials   = Usuario o contraseña incorrectos.
error-account-blocked       = Tu cuenta está bloqueada. Contacta con el administrador.
error-account-pending       = Verifica tu dirección de email antes de iniciar sesión.
error-account-locked        = Demasiados intentos fallidos. Inténtalo de nuevo más tarde.
error-password-mismatch     = Las contraseñas no coinciden.
error-password-too-short    = La contraseña debe tener al menos { $n } caracteres.
error-invalid-timezone      = Zona horaria no válida.
error-invalid-language      = Idioma no válido.
error-invalid-theme         = Tema no válido.
error-about-too-long        = El texto «Sobre mí» no puede superar { $n } caracteres.
error-display-name-too-long = El nombre visible no puede superar { $n } caracteres.
error-username-taken        = Este nombre de usuario ya está en uso o se confunde con otro existente.
error-invalid-username      = El nombre de usuario no es válido: revisa las reglas indicadas bajo el campo.
error-invalid-email         = La dirección de correo no es válida.
error-email-taken           = Esta dirección de email ya está registrada.
error-token-invalid         = Este enlace no es válido o ha caducado.
error-current-password      = La contraseña actual no es correcta.
error-internal              = Se ha producido un error inesperado. Inténtalo de nuevo.

# **< Estados de cuenta >**

status-active  = Activo
status-blocked = Bloqueado
status-pending = Pendiente de verificación de email

# **< Administración: títulos de página >**

title-admin-users            = Usuarios
title-admin-user-new         = Nuevo usuario
title-admin-user-edit        = Editar usuario
title-admin-user-view        = Ver usuario
title-admin-user-password    = Restablecer contraseña
title-admin-roles            = Roles
title-admin-role-new         = Nuevo rol
title-admin-role-edit        = Editar rol
title-admin-role-view        = Ver rol
title-admin-role-permissions = Permisos del rol
title-admin-permissions      = Permisos
title-user-details           = Datos del usuario
title-role-details           = Datos del rol

# **< Administración: descripciones de página >**

description-admin-users       = Gestiona las cuentas de usuario y su acceso.
description-admin-roles       = Gestiona los roles y sus permisos.
description-admin-permissions = Consulta el catálogo de permisos por extensión.

# **< Administración: columnas de tabla >**

col-username     = Usuario
col-email        = Email
col-display-name = Nombre visible
col-roles        = Roles
col-status       = Estado
col-actions      = Acciones
col-machine-name = Nombre técnico
col-label        = Etiqueta
col-type         = Tipo
col-users-count  = Usuarios

# **< Administración: etiquetas de campos >**

field-username-admin        = Usuario
field-display-name          = Nombre visible
field-about                 = Sobre mí
field-member-since          = Miembro desde
field-language              = Idioma
field-timezone              = Zona horaria
field-theme                 = Tema
value-theme-site-default    = Tema del sitio: { $theme }
field-machine-name          = Nombre técnico
field-label                 = Etiqueta
field-description           = Descripción
field-weight                = Peso
field-roles                 = Roles
field-is-admin              = Administrador (acceso irrestricto)
field-search-users          = Buscar por usuario, email o nombre...

help-username-strict =
    Entre { $min } y { $max } caracteres: letras sin tildes ni eñes, números, punto, guion o guion
    bajo, empezando por una letra o un número. No se distinguen mayúsculas al iniciar sesión.

help-username-unicode =
    Entre { $min } y { $max } caracteres: letras de cualquier alfabeto (sin mezclarlos), números,
    punto, guion o guion bajo, empezando por una letra o un número. No se distinguen mayúsculas
    al iniciar sesión.

help-username-permissive =
    Entre { $min } y { $max } caracteres: letras de cualquier alfabeto (sin mezclarlos), números,
    espacios simples, punto, guion, guion bajo, apóstrofo o «+», empezando por una letra o un
    número. No se distinguen mayúsculas al iniciar sesión.

help-password-min-length = Al menos { $n } caracteres.

help-machine-name-immutable =
    Sólo minúsculas, dígitos y guiones bajos. No se puede cambiar tras crearlo.

help-admin-roles =
    Esta cuenta es de administrador y tiene todos los permisos. Estos roles se aplicarán si deja de
    serlo.

# **< Administración: botones y enlaces >**

btn-save               = Guardar
btn-create-user        = Nuevo usuario
btn-create-role        = Nuevo rol
btn-delete             = Eliminar
btn-cancel             = Cancelar
btn-edit               = Editar
btn-manage-permissions = Gestionar permisos
btn-reset-password     = Restablecer contraseña
btn-block              = Bloquear
btn-activate           = Activar
btn-grant-admin        = Conceder administrador
btn-revoke-admin       = Revocar administrador
link-back-to-list      = Volver al listado

# **< Administración: confirmaciones y distintivos >**

confirm-delete-role    = ¿Eliminar este rol? Esta acción no se puede deshacer.
confirm-change-status  = ¿Cambiar el estado de esta cuenta?
confirm-grant-admin    = ¿Conceder acceso irrestricto a esta cuenta?
confirm-revoke-admin   = ¿Revocar el acceso irrestricto de esta cuenta?
badge-system-role      = Sistema
badge-admin            = Administrador
badge-no-permissions   = Sin permisos
empty-users-list       = No se han encontrado usuarios.
empty-roles-list       = No se han encontrado roles.
empty-role-permissions = Este rol no tiene permisos.

# **< Administración: mensajes de error >**

error-role-not-found               = Rol no encontrado.
error-role-machine-name-taken      = Este nombre técnico ya está en uso.
error-invalid-machine-name         = El nombre técnico sólo admite minúsculas, dígitos y guiones bajos, hasta { $n } caracteres.
error-invalid-role-label           = La etiqueta no puede quedar vacía ni superar { $n } caracteres.
error-role-locked                  = Este rol es de sistema y no se puede modificar.
error-role-in-use                  = Este rol tiene usuarios asignados y no se puede eliminar.
error-last-administrator           = No se puede quitar al último administrador.
error-cannot-block-self            = No puedes bloquear tu propia cuenta.
error-cannot-modify-own-admin-flag = No puedes conceder ni revocar tu propio acceso irrestricto.
error-user-not-found               = Usuario no encontrado.
error-unknown-permission           = Clave de permiso desconocida.
