Feature: Remove Meals

    Scenario Outline: Remove meals of all kinds in a given period
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
        When I remove all meals in the period <period>
        Then I get the meal records <deleted_records>
        Then the storage, asked to show the meal records in the period <show_period>, returns <remaining_records>

        Examples:
            | period                                | deleted_records                                                                                                                                                                                                                                                                                                                                                                               | show_period | remaining_records                                                                                                                                                                                                                                                                                                                                                                             |
            | March 18, 2025 through March 24, 2025 | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1742256000, gratin à la m. o.                                                                                                                                                                                                    | March 2025  | 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o.; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese                                                                                                                                                                                             |
            | March 11, 2025 through March 17, 2025 | 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o.; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese                                                                                                                                                                                             | March 2025  | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1742256000, gratin à la m. o.                                                                                                                                                                                                    |
            | March 2025                            | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1742256000, gratin à la m. o.; 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o.; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese | March 2025  |                                                                                                                                                                                                                                                                                                                                                                                               |
            | February 2025                         |                                                                                                                                                                                                                                                                                                                                                                                               | March 2025  | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1742256000, gratin à la m. o.; 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o.; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese |

    Scenario Outline: Remove meals of one kind in a given period
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
        When I remove the meal <meal> in the period <period>
        Then I get the meal records <deleted_records>
        Then the storage, asked to show the meal records in the period <show_period>, returns <remaining_records>

        Examples:
            | meal              | period        | deleted_records                                                                             | show_period | remaining_records                                                                                                                                                                                                                                                                                                                                                                             |
            | tortelloni        | March 2025    | 1741996800, tortelloni                                                                      | March 2025  | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1742256000, gratin à la m. o.; 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o.; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese                         |
            | gratin à la m. o. | March 2025    | 1742256000, gratin à la m. o.; 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o. | March 2025  | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese                                                                                              |
            | gratin à la m. o. | February 2025 |                                                                                             | March 2025  | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese                                                                                              |
            | curry             | March 2025    |                                                                                             | March 2025  | 1742774400, spätzle; 1742688000, spätzle; 1742601600, rougailles saucisses; 1742515200, rougailles saucisse; 1742428800, bramboráky; 1742342400, bramboráky; 1742256000, gratin à la m. o.; 1742169600, gratin à la m. o.; 1742083200, gratin à la m. o.; 1741996800, tortelloni; 1741910400, flammkuchen; 1741824000; 1741824000, rinderbraten; 1741737600, bolognese; 1741651200, bolognese |
