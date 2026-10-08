# British English writes the day before the month. Only these keys differ from en-US; the rest
# fall back to en-US.

# Date formats.
date_format_short = { $day }/{ $month }/{ $year_short }
date_format_medium = { $day }/{ $month }/{ $year }
date_format_long = { $day } { $month } { $year }

# Date input (strftime pattern, see en-US).
date_input_format = %d/%m/%Y
date_input_hint = dd/mm/yyyy

# Start/end date precision.
since_long = since { $day } { $month } { $year }
until_long = until { $day } { $month } { $year }
