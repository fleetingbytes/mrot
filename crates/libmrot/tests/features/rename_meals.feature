Feature: Rename Meals

    Scenario Outline: Rename the records of a meal
        Given an in-memory storage with the records
            | date       | meal         |
            | 1741996800 | spaghetti    |
            | 1741910400 | spaghetti    |
            | 1741824000 | rinderbraten |
            | 1741737600 | spaghetti    |
            | 1741651200 | spaghetti    |
        When I rename the meal <old_name> to <new_name> in the period <period>
        Then I get the meal records <renamed_records>
        Then the storage, asked when <new_name> was consumed, returns <new_name_dates>

        Examples:
            | old_name  | new_name  | period                               | renamed_records                                                                            | new_name_dates                                 |
            | spaghetti | bolognese | None                                 | 1741651200, spaghetti; 1741737600, spaghetti; 1741910400, spaghetti; 1741996800, spaghetti | 2025-03-11, 2025-03-12, 2025-03-14, 2025-03-15 |
            | spaghetti | bolognese | from March 11 through March 12, 2025 | 1741651200, spaghetti; 1741737600, spaghetti                                               | 2025-03-11, 2025-03-12                         |
            | spaghetti | bolognese | from March 14 through March 15, 2025 | 1741910400, spaghetti; 1741996800, spaghetti                                               | 2025-03-14, 2025-03-15                         |
            | spaghetti | bolognese | from March 12 through March 14, 2025 | 1741737600, spaghetti; 1741910400, spaghetti                                               | 2025-03-12, 2025-03-14                         |
