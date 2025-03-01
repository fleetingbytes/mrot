Feature: Parse Date

    For the user to check and practise understanding how strings will be parsed into dates
    the library shall provide a function which would convert a single date or a date range
    from a string into a collection of dates.

    Scenario Outline: Parse Date
        When I parse the date "<text_date>"
        Then the parse result is <naive_dates>

        Examples:
            | text_date                                 | naive_dates                        |
            | 2025-03-01                                | 2025-03-01                         |
            | 12 hours before and after noon 2025-03-01 | 2025-03-01                         |
            | 13 hours before and after noon 2025-03-01 | 2025-02-28, 2025-03-01, 2025-03-02 |
            | from 2025-02-25 until 2025-03-01          | 2025-02-28, 2025-03-01             |
            | from 2025-02-25 until 2025-03-02          | 2025-02-28, 2025-03-01, 2025-03-02 |
