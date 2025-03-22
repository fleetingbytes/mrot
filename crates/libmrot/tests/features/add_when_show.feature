Feature: Add meal, When meal, Show date range

    Scenario Outline: Add meal, show when meal
        Given an empty in-memory storage
        When I add the meal <meal> on the date <text_date> to the storage
        Then the storage, asked when <meal> was recorded, returns <naive_dates>

        Examples:
            | meal      | text_date                                                            | naive_dates                                                |
            | chicken   | 2025-02-23                                                           | 2025-02-23                                                 |
            | weasel    | 12 hours before and after noon on February 26th, 2025                | 2025-02-26                                                 |
            | tuna      | 1 second before February 26th, 2025                                  | 2025-02-25                                                 |
            | ravioli   | from 2025-03-10 through 2025-03-11; 2025-03-03                       | 2025-03-03, 2025-03-10, 2025-03-11                         |
            | spaghetti | from 2025-03-10 through 2025-03-11; from 2025-02-01 until 2025-02-04 | 2025-02-01, 2025-02-02, 2025-02-03, 2025-03-10, 2025-03-11 |

     Scenario Outline: Add meal on several dates, show when meal
        Given an empty in-memory storage
        When I add the meal <meal> on the dates <text_dates> to the storage
        Then the storage, asked when <meal> was recorded, returns <naive_dates>

        Examples:
            | meal    | text_dates                         | naive_dates                        |
            | chicken | 2025-02-23; 2025-02-24; 2025-02-25 | 2025-02-23, 2025-02-24, 2025-02-25 |

     Scenario Outline: Add meal on several dates, ask what meals between dates
        Given an empty in-memory storage
        When I add the meal <meal> on the dates <text_dates> to the storage
        Then the storage, asked for the dates <show_range> returns <meals>

        Examples:
            | meal    | text_dates                         | show_range                         | meals                     |
            | chicken | 2025-02-23                         | 2025-02-23                         | chicken                   |
            | chicken | 2025-02-23; 2025-02-24; 2025-02-25 | 2025-02-23                         | chicken                   |
            | chicken | 2025-02-23; 2025-02-24; 2025-02-25 | from 2025-02-23 to 2025-02-25      | chicken, chicken          |
            | chicken | 2025-02-23; 2025-02-24; 2025-02-25 | from 2025-02-23 through 2025-02-25 | chicken, chicken, chicken |
