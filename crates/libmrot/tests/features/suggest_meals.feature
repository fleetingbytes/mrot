Feature: Suggest Meals

    Scenario Outline:
        Given an in-memory storage with the records
            | date       | meal               |
            | 1741820400 | rinderbraten       |
            | 1741734000 | bolognese          |
            | 1741647600 | bolognese          |
            | 1741561200 | rougaille saucisse |
            | 1741474800 | rougaille saucisse |
            | 1741388400 | pizza              |
            | 1741302000 | pizza              |
            | 1741215600 | chilli con carne   |
            | 1741129200 | chilli con carne   |
            | 1741042800 | pork filet         |
            | 1740956400 | pork filet         |
            | 1740870000 | lentils            |
            | 1740783600 | lentils            |
            | 1740697200 | confit de canard   |
            | 1740610800 | confit de canard   |
            | 1740524400 | kassler            |
            | 1740438000 | kassler            |
            | 1740351600 | meat balls         |
            | 1740265200 | meat balls         |
            | 1740178800 | hamburgers         |
            | 1740092400 | hamburgers         |
            | 1740006000 | chicken curry      |
            | 1739919600 | chicken curry      |
            | 1739833200 | chicken            |
            | 1739746800 | chicken            |
            | 1739660400 | chineese noodles   |
            | 1739574000 | chineese noodles   |
            | 1739487600 | šunkafleky         |
            | 1739401200 | šunkafleky         |
            | 1739314800 | gratin à la M. O.  |
            | 1739228400 | gratin à la M. O.  |
        When I ask for <number> meal suggestions, ignoring <ignore_list> and look-ahead <look_ahead>
        Then I get the meal records <records>

        Examples:
            | number | ignore_list                                     | look_ahead | records                                  |
            | 3      | confit de canard, gratin à la M. O., šunkafleky | 2025-03-13 | chineese noodles, chicken, chicken curry |
