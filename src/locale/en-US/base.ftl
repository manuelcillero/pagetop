# Breadcrumb component.
breadcrumb_label = Breadcrumb navigation

# Button component.
button_save = Save
button_save_and_new = Save & New
button_save_and_new_notice = Saved. You can create another record.
button_edit = Edit
button_delete = Delete
button_cancel = Cancel
button_back = Back

# Dialog component.
dialog_close = Close dialog

# Dropdown component.
dropdown_toggle = Toggle dropdown
dropdown_default_title = Dropdown

# Form components.
field_required = This field is required

select_language_site_default = Use the site language: { $language }
select_language_placeholder = Choose a language...

select_theme_site_default = Use the site theme: { $theme }
select_theme_placeholder = Choose a theme...

select_timezone_site_default = Use the site time zone: { $timezone }
select_timezone_placeholder = Choose a time zone...
select_timezone_current = Current time zone

timezone_region_africa = Africa
timezone_region_america = America
timezone_region_antarctica = Antarctica
timezone_region_arctic = Arctic
timezone_region_asia = Asia
timezone_region_atlantic = Atlantic
timezone_region_australia = Australia
timezone_region_europe = Europe
timezone_region_indian = Indian Ocean
timezone_region_pacific = Pacific
timezone_region_etc = Other

# Intro component.
intro_default_title = Hello, world!
intro_default_slogan = <strong>{ $app }</strong> is up and running
intro_default_button = <strong>Meet PageTop</strong> Modular web solutions in Rust

intro_release_label = Released
intro_license_label = License

intro_text1 = PageTop is <strong>an opinionated Rust web development framework</strong> designed to build modular, extensible, and configurable web solutions.
intro_text2 = PageTop brings back the essence of the classic web: it renders HTML on the server (SSR) with <em>HTML-first</em> components, CSS and JavaScript, <strong>with the performance and security of Rust</strong>.

intro_code = Code
intro_have_fun = All you need is an idea

# ListFilter component.
list_filter_submit = Filter
list_filter_reset = Clear
list_filter_pending = There are filters not applied yet

# ListView component.
list_view_summary = { $count ->
    [0] No results
    [one] 1 result
   *[other] { $count } results
}

# Navbar component.
navbar_toggle = Toggle navigation

# Pager component.
pager_aria_label = Page navigation
pager_previous_label = Previous
pager_previous_aria_label = Previous page
pager_next_label = Next
pager_next_aria_label = Next page
pager_goto_label = Jump to page
pager_goto_button = Go
pager_summary = Showing { $first }-{ $last } of { $total }

# PoweredBy component.
poweredby_pagetop = Powered by { $pagetop_link }

# Table component.
table_empty = No data to display

# File uploads (web::Upload and web::Download).
upload_too_large = "{ $file }" exceeds the maximum size of { $size } per file.
upload_extension_not_allowed = "{ $file }" is not an allowed file type ({ $allowed }).
upload_too_many_files = { $limit ->
    [one] Only one file can be sent at a time.
   *[other] Only { $limit } files can be sent at a time.
}
upload_fields_too_large = The form fields exceed the maximum size of { $size }.
upload_body_too_large = The submission exceeds the maximum size of { $size }.
upload_not_found = The file does not exist or is no longer available.
upload_invalid = The form submission is not valid. Please try again.
upload_failed = The file could not be processed. Please try again later.
