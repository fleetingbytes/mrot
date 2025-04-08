# Meal Rotator

Helps you to rotate through the meals you cook by suggesting what to cook next.

## Why mrot exists

The Meal Rotator, or *mrot* for short, is a tool to record the kinds of meals you and your family consume and the dates when you consume them. When you ask mrot to give you suggestions what to cook next, the meals which you haven't had in the longest time become the likely candidates but your food preferences and meal plans for the future are also considered. Also mrot limits the number suggested meals to as few or as many as you feel comfortable with.

## Quick Start

By using the *add* subcommand you tell mrot what meal your family consumed on what date. The same subcommand enables you to plan some meals for the days to come, if you wish to do so.

### Examples:

Record that you had spaghetti today. The cli option `--date` defaults to `"today"`, so there is no need to explicitly use it here:
```sh
$ mrot add spaghetti
```

Record that you are going to have spaghetti tomorrow:
```sh
$ mrot add spaghetti --date tomorrow
```

To remove meal records, use the *remove* subcommand:
```sh
$ mrot remove "this week"
```

### Getting Meal Suggestions

To demonstrate how mrot suggests meals, we will need to have some data recorded to work with:
```sh
$ mrot add spaghetti --date "from March 1 through March 2"
$ mrot add "meat balls" --date "from March 3 through March 4"
$ mrot add pizza --date "March 5"
$ mrot add steak --date "March 6"
$ mrot add "lentils and wieners" --date "from March 8 through March 9"

$ # assuming today is March 9
$ # plan to have meat balls on March 11
$ mrot add "meat balls" --date "one day after tomorrow"

$ # let's see what meal we could have next
$ mrot what
spaghetti
pizza
steak
```

Notice how *meat balls* were not suggested even though you haven't had them for a longer time than a pizza or a steak. That is because you already planned them in the near future.


When you run the `what` subcommand mrot tries to suggest you the meals which you have not consumed for the longest time. If a meal from long ago matches a meal planned in the near future (by default in the next twelve days starting tomorrow), it is not suggested in order to avoid having the same meal again too soon. This is called the *ignore period* option and you can configure it or disable it entirely. Independent of this you can pass the names of any meals which you do not want to be suggested, see the *ignore* option below.

The procedure which mrot runs internally is something like this:

* for each unique recorded meal, look up the date when it was last consumed
* filter out the meals which are on the ignore list
* filter out the meals which are planned and recorded in advance within the ignore period
* limit the number of suggestions according to your configuration or the CLI option

## Feature Ovewiew

### Record or Plan Meals

* `mrot add spaghetti` records that you've had spaghetti today ("today" is the default date for the add subcommand)
* `mrot add pizza --date yesterday` records that you've had a pizza yesterday
* `mrot add steak --date 2024-02-10 --date "next Saturday through Sunday"` records that you've had a steak on February 10th 2024 and that you plan it for the next Saturday and Sunday
* `mrot add carp --date "this Monday to Wednesday"` records that you had carp this Monday and Tuesday (*sic!*, the date range `X to Y` excludes `Y`).

### Parsing Date Expressions

Mrot uses [two_timer's][two-timer] parse function to translate natural-language strings to dates or a date range, with a few twists when it comes to date ranges. You can look up the exact rules and examples with which mrot is tested in the corresponding [feature file][feature-file].

In order for you to check whether a certain date expression can be parsed or how it is parsed, mrot provides the dedicated `parse-date` subcommand.

* `mrot parse-date "17th of March 2025"`
* `mrot parse-date "yesterday and today"` (This cannot be parsed. Use `from yesterday through today` or two separate dates, e.g. `mrot add pizza --date "yesterday" --date "today"`)
* `mrot parse-date "from yesterday until today"`
* `mrot parse-date "from yesterday through today"`
* `mrot parse-date "one day before and after today"` (This actually includes tomorrow, unlike two_timer's parse result.)

#### Output Dates as Unix Timestamps

A more developer-oriented option allows to output the parsed dates as Unix timestamps

```sh
$ mrot parse-date --output-timestamp "April 6th through April 10th, 2025"
[1743897600, 1743984000, 1744070400, 1744156800, 1744243200]
```

### Getting Cooking Suggestions

* `mrot what` will suggest some meals to cook, taking your past, planned and ignored meals into account
* `mrot what --ignore-period "from tomorrow to three days after tomorrow"` same as above, but override the configured ignore period
* `mrot what --no-ignore-period` same as above but no ignore period, thus potentially suggesting any meals you may have planned to cook in the near future.
* `mrot what --ignore liver --ignore salad` same as above, ignoring liver and salad (this supersedes your regular ignore list from your mrot configuration)
* `mrot what --no-ignore` same as above, not taking the ignore list from your mrot configuration into account
* `mrot what --number 5` same as above, overriding the regular number of meals to suggest. The given number is the upper limit. If you have not recorded enough meals to reach this number of suggestions, mrot will suggest less.

#### Random Meal

* `mrot random` will show you one random meal from all of your records. This can also pick the meals from the ignore list or the ones planned for the future. The date when this meal was last cooked does not play any role.

### Browsing Meals

#### Recent Past and Near Future

* `mrot show` will show the past and next planned meals according to the configuration
* `mrot show "from last Tuesday to next Monday"` will show the recorded or planned meals in the given time range
* `mrot show "this week"` will show the past and future meals in this week

#### A Meal's Consumption Past and Future

* `mrot when "spaghetti"` will show all past and future dates of your spaghetti records.

#### Unique Meals

* `mrot unique` will show you all unique meal names used in your records

### Managing Recorded Meals

* `mrot remove "from last week to next week"` will remove all meals in the specified time range
* `mrot remove "from last month to the end of this month" --meal "tomato soup"` will remove the specified meal in the specified time range

### Renaming Meals

* `mrot rename "spaghetti" "spaghetti bolognese"` will rename all records of *spaghetti* to *spaghetti bolognese*
* `mrot rename "spaghetti bolognese" "spaghetti alla carbonara" --date "two weeks ago through today"` will rename records of *spaghetti bolognese* to *spaghetti alla carbonara* if their date happens to be in the period from *two weeks ago through today*

### Configuring Mrot

* `mrot config set what number 5` will configure mrot to suggest five oldest meals (default: 3)
* `mrot config set what ignore-period "from tomorrow through 5 days after tomorrow"` will configure mrot avoid suggesting meals found in any records in this period
* `mrot config set show "from the day before yesterday until tomorrow"` will configure mrot to show the meals planned for the specified range
* `mrot config get what number` will show how many meals is mrot configured to suggest
* `mrot config get what ignore-period` will show the period meals from which are not to be be suggested
* `mrot config get show` will show the time in which mrot-show will show meals
* `mrot config ignore add liver` will add liver to the ignore list
* `mrot config ignore remove salad` will remove salad from the ignore list
* `mrot config ignore show` will list the ignored meals
* `mrot config ignore clear` will remove everything from the ignore list

#### Restoring Default Configuration

To restore the default configuration, simply delete your config file. Mrot will create a new one next time it runs.

### Show Paths to Mrot's Data Files

* `mrot path config` will show the path to the config file
* `mrot path records` will show the path to the records file
* `mrot path log` will show the path to the log file

### Command Completions

* `mrot generate zsh` will generate shell completions for zsh (completions for Bash, Elvish, Fish, Nushell, PowerShell are also available)

## Non-Goals

### Handling of More Than One Meal Per Day

Mrot is intended to record only the prime meal of the day (lunch). This is because in my family the breakfasts and dinners are routinely the same and change only occasionally. You can record or plan multiple meals on a single day, but they are all equivalent. Mrot will not distinguish if a meal was a breakfast, lunch, or dinner. In queries which limit the number of meals shown, e.g. `mrot show --number 3`, meals on the same date are of equal importance because mrot ranks the meals by their date. Both would outrank younger records. If in the composition of a listing of meals the number of items is limited, two equally dated meals fighting for an item slot will outrank each other unpredictably.

### Recipe Management

Mrot does not manage your recipes and does not want to link the meals you record to a recipe.

[two-timer]: https://docs.rs/two_timer/latest/two_timer/
[feature-file]: https://github.com/fleetingbytes/mrot/tree/master/crates/libmrot/tests/features/parse_date.feature
