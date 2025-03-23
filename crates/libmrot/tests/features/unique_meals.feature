Feature: Uniqu Meals

    Scenario Outline:
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
        When I ask for unique meals
        Then I get the meal records <records>

        Examples:
            | records                                                                                                                                                                                                       |
            | 1741737600, bolognese; 1741824000, rinderbraten; 1741910400, flammkuchen; 1741996800, tortelloni; 1742256000, gratin à la m. o.; 1742428800, bramboráky; 1742601600, rougailles saucisse; 1742774400, spätzle |


