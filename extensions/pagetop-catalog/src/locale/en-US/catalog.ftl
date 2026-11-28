extension-name = Catalogs
extension-description = Administration of the catalogs of the application: list, create, edit, deactivate, delete and history.

# **< Permissions >*********************************************************************************
permission-group = Catalogs
permission-view = View { $catalog }
permission-manage = Manage { $catalog }

# **< Fields >**************************************************************************************
field-code = Code
field-name = Name
field-weight = Order
field-active = Active
field-translations = Translations of the name

help-code = Required. Unique in the catalog, without spaces. It cannot be changed once created.
help-code-fixed = The code cannot be changed once created.
help-name = Required. The name shown to users.
help-weight = Optional. Position in lists: lower numbers first. 0 if left empty.
help-translations = Optional. The name in other languages; where it is empty, the name above is used.

value-yes = Yes
value-no = No
# A deactivated record, only offered because the record being edited already uses it.
option-inactive = { $name } (inactive)

# **< List >****************************************************************************************
filter-search = Search
placeholder-search = Code or name (* wildcard)
placeholder-search-name = Name (* wildcard)
filter-status = Status
status-active = Active
status-inactive = Inactive
filter-all = All
status-all = All

col-actions = Actions
empty-list = There are no records.

btn-new = New
btn-view = View
btn-deactivate = Deactivate
btn-reactivate = Reactivate
btn-history = History

confirm-delete = Delete "{ $name }"? This cannot be undone.

# **< Pages >***************************************************************************************
title-new = New record: { $catalog }
title-edit = Edit record: { $catalog }
title-view = Record: { $catalog }
title-history = History of { $record }

# **< Results >*************************************************************************************
done-created = The record has been created.
done-updated = The changes have been saved.
done-deactivated = The record has been deactivated.
done-reactivated = The record has been reactivated.
done-deleted = The record has been deleted.

error-internal = The operation could not be completed. Try again later.
error-not-found = The record does not exist.
error-duplicate = Another record already has the same { $field1 }.
error-duplicate-combination = Another record already has the same combination of { $count ->
    [2] { $field1 } and { $field2 }
   *[other] { $field1 }, { $field2 } and { $field3 }
}.
error-duplicate-any = Another record already has some of these values.
error-not-a-decimal = { $field } must be a number.
error-invalid-characters = { $field } has characters that cannot be stored.
error-conflict = Someone else has changed this record since you opened it. These are the current values; make your changes again.
error-in-use = The record is in use and cannot be deleted. You can deactivate it instead.
error-required = { $field } is required.
error-too-long = { $field } can have at most { $max } characters.
error-code-spaces = The code cannot contain spaces.
error-not-a-number = { $field } must be a whole number.
error-out-of-range = { $field } is out of the allowed range.
error-invalid-option = { $field } has a value that is not allowed.

# **< History >*************************************************************************************
col-when = When
col-who = Who
col-action = Action
col-changes = Changes
empty-history = There are no recorded changes.

action-create = Created
action-update = Modified
action-deactivate = Deactivated
action-reactivate = Reactivated
action-delete = Deleted
action-import = Imported

# **< CSV >*****************************************************************************************
btn-export = Export CSV
btn-import = Import CSV
btn-preview = Preview
btn-other-file = Another file

title-import = Import CSV: { $catalog }
field-import-file = CSV file
help-import-file = Required. A header row and one row per record, separated by commas or semicolons, in UTF-8. Columns: { $columns }. Required: { $required }; a missing column keeps the current values. Columns that identify each record: { $keys }; new records are created and existing ones are updated. { $limits } Export the catalog from its list to get a file with the right columns.
help-import-limits = At most { $rows } rows and { $size } per file.
help-import-limits-rows = At most { $rows } rows per file.

col-record = Record
col-line = Line
col-result = Result
import-create = Will be created.
import-update = Will change: { $columns }.
import-unchanged = No changes.
import-summary = { $created } to create, { $updated } to change, { $unchanged } without changes, { $invalid } with errors.{ $invalid ->
    [0] {""}
   *[other] {" "}Correct the file and upload it again: nothing is imported while a row has errors.
}
done-imported = Import finished: { $created } created, { $updated } changed.

import-error-encoding = The file is not in UTF-8. Save it again as "CSV UTF-8".
import-error-empty = The file is empty.
import-error-quotes = The file has quotes that are not closed, starting at line { $line }.
import-error-too-many = The file has too many rows: at most { $max } can be imported at once.
import-error-too-large = The file is too large: at most { $max } can be imported at once.
import-error-unknown-column = Unknown column "{ $column }".
import-error-missing-column = Missing column "{ $column }".
import-error-repeated = The code is repeated in line { $line }.
import-error-changed = The data have changed since the preview. Upload the file again.
