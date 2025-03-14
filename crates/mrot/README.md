# Meal Rotator

Helps you to rotate through the meals you cook by suggesting what to cook next.

## Why mrot exists

We regularly cook meals from a small repertoir of recipes. Yet, when deciding what to cook next, we struggle to remember which meals we didn't have in a while. The Meal Rotator, or *mrot* for short, records the dates when you last cooked spaghetti, pizza, or had a steak. The meals not cooked in the longest time become likely candidates to suggest to cook next.

## Quick Start

By using the `add` subcommand you tell mrot what meal you cooked on what date. The same subcommand enables you to plan some meals for the days to come, if you wish to do so.

```sh
$ mrot add spaghetti --date "from March 1 through March 2"
$ mrot add "meat balls" --date "from March 3 through March 4"
$ mrot add pizza --date "March 5"
$ mrot add steak --date "March 6"
$ mrot add "lentils and wieners" --date "from March 8 through March 9"
$ # assuming today is March 9
$ # plan to have meat balls on March 11
$ mrot add "meat balls" --date "one day after tomorrow"
$ mrot what
spaghetti
pizza
steak
```

Notice how meat balls were not suggested even though you haven't had them for a longer time than a pizza or a steak. That is because they are already on your cooking plan for the days to come.

### Getting Meal Suggestions

When you run the `what` subcommand mrot tries to suggest you the meals which you have not consumed for the longest time. However, any meal which you might have already recorded (planned) for tomorrow is suggested first. Also, if a meal from long ago matches a meal planned in the near future (in the eleven days following tomorrow by default), it is not suggested to avoid having the same meal again a few days later (this is assuming you would stick to your plan).

The procedure which mrot runs internally is something like this:

* if you have any meal planned for tomorrow, suggest that

otherwise:

* get a list of unique meals from your records
* filter out the meals from the ignore list
* filter out the meals planned in the look-ahead period (by default from one day after tomorrow through 11 days after tomorrow)
* look up the last date when each of the remaining meals were cooked and suggest those with the earliest dates
* limit the number of suggestions according to mrot's configuration or the CLI option


## Feature Ovewiew

### Record or Plan Meals

* `mrot add spaghetti` records that you've had spaghetti today
* `mrot add pizza --date yesterday` records that you've had a pizza yesterday
* `mrot add steak --date 2024-02-10 --date "next Saturday through Sunday"` records that you've had a steak on February 10 2024 and that you plan it for the next Saturday and Sunday"
* `mrot add carp --date "this Monday to Wednesday"` records that you had carp this Monday and Tuesday (*sic!*, the date range `X to Y` excludes `Y`).

### Parsing Date Expressions

Mrot uses [two_timer's][two-timer] parse function to translate natural-language strings to dates, with a few twists when it comes to date ranges. You can look up the exact rules and examples with which mrot is tested in the corresponding [feature file][feature-file].

In order for you to check whether a certain date expression can be parsed or how it is parsed, mrot provides the dedicated `parse-date` subcommand.

* `mrot parse-date "yesterday and today"` (This cannot be parsed. Use `from yesterday through today` or two separate dates, e.g. `mrot add pizza --date "yesterday" --date "today"`)
* `mrot parse-date "from yesterday until today"`
* `mrot parse-date "from yesterday through today"`
* `mrot parse-date "one day before and after today"` (This actually includes tomorrow, unlike two_timer's parse result.)

### Getting Cooking Suggestions

* `mrot what` will suggest some meals to cook, taking your planned and ignored meals into account
* `mrot what --look-ahead 4` same as above, but override the configured look-ahead period to be "one day after tomorrow through 4 days after tomorrow"
* `mrot what --no-look-ahead` same as above, potentially including any meals you may have planned to cook in the near future.
* `mrot what --ignore liver --ignore salad` same as above, ignoring liver and salad (this supersedes your regular ignore list from your mrot configuration)
* `mrot what --no-ignore` same as above, not taking the ignore list from your mrot configuration into account
* `mrot what --number 5` same as above, overriding the regular number of meals to show. The given number is the upper limit. If you have not recorded enough meals to reach this number of suggestions, mrot will suggest less.

#### Random Meal

* `mrot random` will show you one random meal from all of your records. This can also pick the meals from the ignore list or the ones planned for the future. The date when this meal was last cooked does not play any role.

### Browsing Meals

* `mrot show` will show the past and next planned meals according to the configuration
* `mrot show "from last Tuesday to next Monday"` will show the recorded or planned meals in the given time range
* `mrot show "this week"` will show the past and future meals in this week

* `mrot when "spaghetti"` will show the past and future dates where spaghetti were recorded

* `mrot unique` will show you all unique meal names used in your records or on the ignore list

### Managing Recorded Meals

* `mrot remove "from last week to next week"` will remove all meals in the specified time range
* `mrot remove "from last month to the end of this month" --meal "tomato soup"` will remove the specified meal in the specified time range

### Configuring Mrot

* `mrot config set what number 5` will configure mrot to suggest five oldest meals (default: 3)
* `mrot config set show "from the day before yesterday until tomorrow"` will configure mrot to show the meals planned for the specified range
* `mrot config get what number` will show how many meals is mrot configured to suggest
* `mrot config get show` will show the time in which mrot-show will show meals
* `mrot config ignore add liver` will add liver to the ignore list
* `mrot config ignore remove salad` will remove salad from the ignore list
* `mrot config ignore show` will list the ignored meals
* `mrot config ignore clear` will remove everything from the ignore list
* `mrot config path` will show the path to the config file

### Command Completions

* `mrot generate zsh` will generate shell completions for zsh (completions for Bash, Elvish, Fish, Nushell, PowerShell are also available)

## Non-Goals

Mrot is intended to record only the prime meal of the day (lunch). This is because in my family the breakfasts and dinners are routinely the same and change only occasionally. You can record or plan multiple meals on a single day, but they are all equivalent. Mrot will not distinguish if a meal was a breakfast, lunch, or dinner. In queries limiting the number of meals shown, e.g. `mrot plan show --number 3`, such meals are in competition with each other and mrot will show only the first of the matches on that day.

[two-timer]: https://docs.rs/two_timer/latest/two_timer/
[feature-file]: https://github.com/fleetingbytes/mrot/tree/master/crates/libmrot/tests/features/parse_date.feature
