## pagetop-user — English (default)

# **< Extension metadata >**

extension_name        = PageTop User
extension_description = User identity, authentication, roles and permissions for PageTop.

# **< Page titles >**

title-login          = Sign in
title-register       = Create account
title-password-reset = Reset password
title-new-password   = Set new password
title-profile        = My profile

# **< Field labels >**

field-username         = Username or email
field-password         = Password
field-email            = Email address
field-confirm-password = Confirm password
field-new-password     = New password
field-remember-me      = Remember me

# **< Buttons and links >**

btn-login           = Sign in
btn-logout          = Sign out
btn-register        = Create account
btn-send-reset-link = Send reset link
btn-set-password    = Change password

link-register        = Create an account
link-forgot-password = Forgot your password?
link-back-to-login   = Back to sign in

# **< Messages >**

msg-password-reset-sent =
    If an account with that email exists, we have sent a reset link.
    Please check your inbox.

# **< Error messages >**

error-invalid-credentials   = Invalid username or password.
error-account-blocked       = Your account is blocked. Please contact the administrator.
error-account-pending       = Please verify your email address before signing in.
error-account-locked        = Too many failed attempts. Please try again later.
error-password-mismatch     = Passwords do not match.
error-password-too-short    = Password must be at least { $n } characters.
error-username-taken        = This username is already taken.
error-email-taken           = This email address is already registered.
error-token-invalid         = This link is invalid or has expired.
error-internal              = An unexpected error occurred. Please try again.

# **< Account statuses >**

status-active  = Active
status-blocked = Blocked
status-pending = Pending email verification

# **< Admin: page titles >**

title-admin-users            = Users
title-admin-user-new         = New user
title-admin-user-edit        = Edit user
title-admin-user-view        = View user
title-admin-user-roles       = User roles
title-admin-user-password    = Reset password
title-admin-roles            = Roles
title-admin-role-new         = New role
title-admin-role-edit        = Edit role
title-admin-role-view        = View role
title-admin-role-permissions = Role permissions
title-admin-permissions      = Permissions
title-user-details           = User details
title-role-details           = Role details

# **< Admin: page descriptions >**

description-admin-users       = Manage user accounts and access.
description-admin-roles       = Manage roles and their permissions.
description-admin-permissions = Browse the permission catalog by extension.

# **< Admin: table columns >**

col-username     = Username
col-email        = Email
col-display-name = Display name
col-roles        = Roles
col-status       = Status
col-actions      = Actions
col-machine-name = Machine name
col-label        = Label
col-type         = Type
col-users-count  = Users

# **< Admin: field labels >**

field-username-admin = Username
field-display-name   = Display name
field-language       = Language
field-timezone       = Timezone
field-machine-name   = Machine name
field-label          = Label
field-description    = Description
field-weight         = Weight
field-roles          = Roles
field-is-admin       = Administrator (unrestricted access)
field-search-users   = Search by username, email or name...

help-machine-name-immutable =
    Lowercase letters, digits and underscores only. Cannot be changed after creation.

# **< Admin: buttons and links >**

btn-save               = Save
btn-create-user        = New user
btn-create-role        = New role
btn-delete             = Delete
btn-cancel             = Cancel
btn-edit               = Edit
btn-manage-roles       = Manage roles
btn-manage-permissions = Manage permissions
btn-reset-password     = Reset password
btn-block               = Block
btn-activate            = Activate
btn-grant-admin        = Grant administrator
btn-revoke-admin       = Revoke administrator
link-back-to-list      = Back to list

# **< Admin: confirmations and badges >**

confirm-delete-role   = Delete this role? This cannot be undone.
confirm-change-status = Change this account's status?
confirm-grant-admin   = Grant unrestricted access to this account?
confirm-revoke-admin  = Revoke this account's unrestricted access?
badge-system-role     = System
badge-admin           = Administrator
empty-users-list      = No users found.
empty-roles-list      = No roles found.

# **< Admin: error messages >**

error-role-not-found              = Role not found.
error-role-machine-name-taken     = This machine name is already taken.
error-invalid-machine-name        = Machine name may only contain lowercase letters, digits and underscores.
error-role-locked                  = This role is a system role and cannot be modified.
error-role-in-use                 = This role has users assigned and cannot be deleted.
error-last-administrator          = Cannot remove the last administrator.
error-cannot-block-self           = You cannot block your own account.
error-cannot-modify-own-admin-flag = You cannot grant or revoke your own unrestricted access.
error-user-not-found              = User not found.
error-unknown-permission          = Unknown permission key.
