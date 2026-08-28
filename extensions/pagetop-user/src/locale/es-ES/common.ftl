## pagetop-user — Español

# **< Metadatos de la extensión >**

extension_name        = PageTop Usuario
extension_description = Identidad de usuario, autenticación, roles y permisos para PageTop.

# **< Títulos de página >**

title-login          = Iniciar sesión
title-register       = Crear cuenta
title-password-reset = Recuperar contraseña
title-new-password   = Establecer nueva contraseña
title-profile        = Mi perfil

# **< Etiquetas de campos >**

field-username         = Usuario o email
field-password         = Contraseña
field-email            = Dirección de email
field-confirm-password = Confirmar contraseña
field-new-password     = Nueva contraseña
field-remember-me      = Recuérdame

# **< Botones y enlaces >**

btn-login           = Entrar
btn-logout          = Cerrar sesión
btn-register        = Crear cuenta
btn-send-reset-link = Enviar enlace
btn-set-password    = Cambiar contraseña

link-register        = Crear una cuenta
link-forgot-password = ¿Olvidaste tu contraseña?
link-back-to-login   = Volver al inicio de sesión

# **< Mensajes >**

msg-password-reset-sent =
    Si existe una cuenta con ese email, hemos enviado un enlace de recuperación.
    Revisa tu bandeja de entrada.

# **< Mensajes de error >**

error-invalid-credentials   = Usuario o contraseña incorrectos.
error-account-blocked       = Tu cuenta está bloqueada. Contacta con el administrador.
error-account-pending       = Verifica tu dirección de email antes de iniciar sesión.
error-account-locked        = Demasiados intentos fallidos. Inténtalo de nuevo más tarde.
error-password-mismatch     = Las contraseñas no coinciden.
error-password-too-short    = La contraseña debe tener al menos { $n } caracteres.
error-username-taken        = Este nombre de usuario ya está en uso.
error-email-taken           = Esta dirección de email ya está registrada.
error-token-invalid         = Este enlace no es válido o ha caducado.
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
title-admin-user-roles       = Roles del usuario
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

field-username-admin = Usuario
field-display-name   = Nombre visible
field-language       = Idioma
field-timezone       = Zona horaria
field-machine-name   = Nombre técnico
field-label          = Etiqueta
field-description    = Descripción
field-weight         = Peso
field-roles          = Roles
field-is-admin       = Administrador (acceso irrestricto)
field-search-users   = Buscar por usuario, email o nombre...

help-machine-name-immutable =
    Sólo minúsculas, dígitos y guiones bajos. No se puede cambiar tras crearlo.

# **< Administración: botones y enlaces >**

btn-save               = Guardar
btn-create-user        = Nuevo usuario
btn-create-role        = Nuevo rol
btn-delete             = Eliminar
btn-cancel             = Cancelar
btn-edit               = Editar
btn-manage-roles       = Gestionar roles
btn-manage-permissions = Gestionar permisos
btn-reset-password     = Restablecer contraseña
btn-block               = Bloquear
btn-activate             = Activar
btn-grant-admin        = Conceder administrador
btn-revoke-admin       = Revocar administrador
link-back-to-list      = Volver al listado

# **< Administración: confirmaciones y distintivos >**

confirm-delete-role   = ¿Eliminar este rol? Esta acción no se puede deshacer.
confirm-change-status = ¿Cambiar el estado de esta cuenta?
confirm-grant-admin   = ¿Conceder acceso irrestricto a esta cuenta?
confirm-revoke-admin  = ¿Revocar el acceso irrestricto de esta cuenta?
badge-system-role     = Sistema
badge-admin           = Administrador
empty-users-list      = No se han encontrado usuarios.
empty-roles-list      = No se han encontrado roles.

# **< Administración: mensajes de error >**

error-role-not-found              = Rol no encontrado.
error-role-machine-name-taken     = Este nombre técnico ya está en uso.
error-invalid-machine-name        = El nombre técnico sólo admite minúsculas, dígitos y guiones bajos.
error-role-locked                  = Este rol es de sistema y no se puede modificar.
error-role-in-use                 = Este rol tiene usuarios asignados y no se puede eliminar.
error-last-administrator          = No se puede quitar al último administrador.
error-cannot-block-self           = No puedes bloquear tu propia cuenta.
error-cannot-modify-own-admin-flag = No puedes conceder ni revocar tu propio acceso irrestricto.
error-user-not-found              = Usuario no encontrado.
error-unknown-permission          = Clave de permiso desconocida.
