Feature: Add meal, When meal, Show date range

    Scenario Outline: Add meal, show when meal
        Given an empty in-memory storage
        When I add the meal <meal> on the date <text_date> to the storage
        Then the storage, asked when <meal> was consumed, returns <naive_dates>

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
        Then the storage, asked when <meal> was consumed, returns <naive_dates>

        Examples:
            | meal    | text_dates                         | naive_dates                        |
            | chicken | 2025-02-23; 2025-02-24; 2025-02-25 | 2025-02-23, 2025-02-24, 2025-02-25 |

     Scenario Outline: Add meal on several dates, ask what meals between dates
        Given an in-memory storage with the records
            | date       | meal                |
            | 1742774400 | spätzle             |
            | 1742688000 | spätzle             |
            | 1742601600 | rougailles saucisse |
            | 1742515200 | rougailles saucisse |
            | 1742428800 | bramboráky          |
            | 1742342400 | bramboráky          |
            | 1742256000 | gratin à la m. o.   |
            | 1742169600 | gratin à la m. o.   |
            | 1742083200 | gratin à la m. o.   |
            | 1741996800 | tortelloni          |
            | 1741910400 | flammkuchen         |
            | 1741824000 | rinderbraten        |
            | 1741737600 | bolognese           |
            | 1741651200 | bolognese           |
        Then the storage, asked to show the meal records in the period <show_period>, returns <meal_records>

        Examples:
            | show_period                   | meal_records                                                              |
            | 2025-03-11                    | 1741651200, bolognese                                                     |
            | 2025-03-12 through 2025-03-14 | 1741737600, bolognese; 1741824000, rinderbraten; 1741910400, flammkuchen  |
            | 2025-03-22 through 2025-03-24 | 1742601600, rougailles saucisse; 1742688000, spätzle; 1742774400, spätzle |
