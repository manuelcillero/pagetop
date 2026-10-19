## pagetop-user — English (default)

# **< Extension metadata >**

extension_name        = PageTop User
extension_description = User identity, authentication, roles and permissions for PageTop.

# **< Page titles >**

title-login            = Sign in
title-register         = Create account
title-password-reset   = Reset password
title-new-password     = Set new password
title-verify-email     = Confirm your email
title-profile          = My profile
title-profile-edit     = Edit my profile
title-profile-password = Change my password

# **< Field labels >**

field-username         = Username or email
field-password         = Password
field-email            = Email address
field-confirm-password = Confirm password
field-new-password     = New password
field-current-password = Current password
field-remember-me      = Remember me

# **< Buttons and links >**

btn-login           = Sign in
btn-logout          = Sign out
btn-register        = Create account
btn-send-reset-link = Send reset link
btn-set-password    = Change password
btn-verify-email    = Confirm my email
btn-edit-profile    = Edit profile

link-register        = Create an account
link-forgot-password = Forgot your password?
link-back-to-login   = Back to sign in

# **< Account menu >**

menu-account = Account
menu-admin   = Administration
menu-login   = Sign in
menu-profile = Profile

# **< Messages >**

msg-password-reset-sent =
    If an account with that email exists, we have sent a reset link.
    Please check your inbox.

msg-verify-email = Press the button to confirm your email address and activate your account.

msg-verification-sent =
    We have sent you an email to confirm your address.
    Open the link it contains and confirm your address to activate your account.

# **< Emails >**

mail-greeting = Hello { $username },
mail-greeting-plain = Hello,
mail-link-expires =
    { $hours ->
        [one] The link expires in one hour.
       *[other] The link expires in { $hours } hours.
    }

mail-reset-subject = Reset your { $app } password
mail-reset-intro   = We received a request to reset the password of your account. To choose a new one, open this link:
mail-reset-action  = Choose a new password
mail-reset-ignore  = If you did not request it, ignore this email: your password will not change.

mail-verify-subject = Confirm your email for { $app }
mail-verify-intro   = Thank you for signing up. To activate your account, confirm your email address by opening this link:
mail-verify-action  = Confirm my email
mail-verify-ignore  = If you did not create an account, ignore this email.

mail-preview-reset  = Password reset
mail-preview-verify = Email verification on sign-up

# **< Error messages >**

error-invalid-credentials   = Invalid username or password.
error-account-blocked       = Your account is blocked. Please contact the administrator.
error-account-pending       = Please verify your email address before signing in.
error-account-locked        = Too many failed attempts. Please try again later.
error-password-mismatch     = Passwords do not match.
error-password-too-short    = Password must be at least { $n } characters.
error-invalid-timezone      = Invalid timezone.
error-invalid-language      = Invalid language.
error-invalid-theme         = Invalid theme.
error-about-too-long        = The "About me" text must be at most { $n } characters.
error-display-name-too-long = The display name must be at most { $n } characters.
error-username-taken        = This username is already taken or can be mistaken for an existing one.
error-invalid-username      = The username is not valid: check the rules shown below the field.
error-invalid-email         = The email address is not valid.
error-email-taken           = This email address is already registered.
error-token-invalid         = This link is invalid or has expired.
error-current-password      = The current password is incorrect.
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

field-username-admin        = Username
field-display-name          = Display name
field-about                 = About me
field-member-since          = Member since
field-language              = Language
field-timezone              = Timezone
field-theme                 = Theme
value-theme-site-default    = Site theme: { $theme }
field-machine-name          = Machine name
field-label                 = Label
field-description           = Description
field-weight                = Weight
field-roles                 = Roles
field-is-admin              = Administrator (unrestricted access)
field-search-users          = Search by username, email or name...

help-username-strict =
    { $min } to { $max } characters: unaccented letters, digits, period, hyphen or underscore,
    starting with a letter or digit. Case is ignored when signing in.

help-username-unicode =
    { $min } to { $max } characters: letters of any alphabet (without mixing them), digits, period,
    hyphen or underscore, starting with a letter or digit. Case is ignored when signing in.

help-username-permissive =
    { $min } to { $max } characters: letters of any alphabet (without mixing them), digits, single
    spaces, period, hyphen, underscore, apostrophe or "+", starting with a letter or digit. Case is
    ignored when signing in.

help-password-min-length = At least { $n } characters.

help-machine-name-immutable =
    Lowercase letters, digits and underscores only. Cannot be changed after creation.

help-admin-roles =
    This is an administrator account with every permission. These roles will apply if it stops
    being one.

# **< Admin: buttons and links >**

btn-save               = Save
btn-create-user        = New user
btn-create-role        = New role
btn-delete             = Delete
btn-cancel             = Cancel
btn-edit               = Edit
btn-manage-permissions = Manage permissions
btn-reset-password     = Reset password
btn-block              = Block
btn-activate           = Activate
btn-grant-admin        = Grant administrator
btn-revoke-admin       = Revoke administrator
link-back-to-list      = Back to list

# **< Admin: confirmations and badges >**

confirm-delete-role    = Delete this role? This cannot be undone.
confirm-change-status  = Change this account's status?
confirm-grant-admin    = Grant unrestricted access to this account?
confirm-revoke-admin   = Revoke this account's unrestricted access?
badge-system-role      = System
badge-admin            = Administrator
badge-no-permissions   = No permissions
empty-users-list       = No users found.
empty-roles-list       = No roles found.
empty-role-permissions = This role has no permissions.

# **< Admin: error messages >**

error-role-not-found               = Role not found.
error-role-machine-name-taken      = This machine name is already taken.
error-invalid-machine-name         = Machine name may only contain lowercase letters, digits and underscores, up to { $n } characters.
error-invalid-role-label           = The label must not be blank or longer than { $n } characters.
error-role-locked                  = This role is a system role and cannot be modified.
error-role-in-use                  = This role has users assigned and cannot be deleted.
error-last-administrator           = Cannot remove the last administrator.
error-cannot-block-self            = You cannot block your own account.
error-cannot-modify-own-admin-flag = You cannot grant or revoke your own unrestricted access.
error-user-not-found               = User not found.
error-unknown-permission           = Unknown permission key.
