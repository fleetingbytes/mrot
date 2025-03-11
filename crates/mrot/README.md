# Meal Rotator

Helps you to rotate through the meals you cook by suggesting what to cook next.

## Why mrot exists

We regularly cook meals from a small repertoir of recipes. Yet, when deciding what to cook next, we struggle to remember which meals we didn't have in a while. The Meal Rotator, or *mrot* for short, records the dates when you last cooked spaghetti, pizza, or had a steak. The meals not cooked in the longest time become likely candidates to suggest to cook next.

## How does mrot decide what to suggest?

By using the `add` subcommand you tell mrot what meal you cooked on what date. The same subcommand enables you to plan some meals for the days to come, if you wish to do so. You also maintain two ignore-lists in mrot's configuration. One is a static ignore list with meals you wouldn't like to cook in the forseeable future (`config ignore`). Another is a dynamic ignore list with meals which you have planned to cook in the next few days (`config look-ahead`). That's the part you do.

Then you can run the `what` subcommand, mrot looks through the data you recorded and does the following:

* if you have any meal planned for tomorrow, suggest that

otherwise:

* get a list of unique meals
* filter out the meals from the ignore list
* filter out the meals planned for the next few days
* look up the last date when each of the remaining meals were cooked and suggest some of those with the earliest dates

## Usage

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

### Get Cooking Suggestions

* `mrot what` will show you some meals which you haven't had for the longest time
* `mrot what --number 5` will show you (at most) five meals which you haven't had for the longest time
* `mrot what --ignore liver --ignore salad` will show you will show you some meals which you haven't had for the longest time, ignoring liver and salad (this supersedes your usual ignore list from your mrot configuration)
* `mrot what --no-look-ahead` will show you will show you some meals which you haven't had for the longest time, ignoring any meals you might have planned to cook in the near future.
* `mrot what --no-ignore` will show you will show you some meals which you haven't had for the longest time, not taking the ignore list from your mrot configuration into account

* `mrot random` will show you one random meal from your past or planned meals. This can also pick the meals from the ignore list or the ones planned for the future.

### Browsing Meals

* `mrot show` will show the past and next planned meals according to the configuration
* `mrot show "from last Tuesday to next Monday"` will show the recorded or planned meals in the given time range
* `mrot show "this week"` will show the past and future meals in this week

* `mrot when "spaghetti"` will show the past and future dates where spaghetti were recorded

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

### Command Completion

* `mrot generate zsh` will generate shell completion for zsh (completions other shells available, too)

## Non-Goals

Mrot is intended to record only the prime meal of the day (lunch). This is because in my family the breakfasts and dinners are routinely the same and change only occasionally. You can record or plan multiple meals on a single day, but they are all equivalent. Mrot will not distinguish if a meal was a breakfast, lunch, or dinner. In queries limiting the number of meals shown, e.g. `mrot plan show --number 3`, such meals are in competition with each other and mrot will show only the first of the matches on that day.

[two-timer]: https://docs.rs/two_timer/latest/two_timer/
[feature-file]: https://github.com/fleetingbytes/mrot/tree/master/crates/libmrot/tests/features/parse_date.feature
