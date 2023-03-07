# `git-popular`

A command-line tool that generates git commits in your repo to make it (and you?) look popular and regularily coding.

## How To Use

1. Make sure you have [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) installed and setup (set your email address and name for example).
2. Create your git repo where you want to generate commits. `git init`.
3. Download the app, it's command line based.
4. Give it a run then push the results.
5. Profit

### Example:
   ```bash
   mkdir fake-history
   cd fake-history
   git init
   git remote add origin git@github.com:<USERNAME>/fake-history.git
   ~/Downloads/git-popular --start 2020-01-01 --end 2023-03-02 --dir . --monday 5 --tuesday 5 --wednesday 6 --thursday 5 --friday 9
   git push -u origin master
   ```


## Support This Project
I don't want your money, but you can still support it by making PRs to make it good-er.


## Arguments
These are all set using as `--argument-name VALUE`

### `--start` and `--end` **(Required)**
In `YYYY-MM-DD` format, set the date to start and stop generating commits.

EG: `--start 2021-02-03` and `--end 2023-03-04` 

### `--dir` **(Required)**
Set the working directory.

EG: `--dir /home/username/projects/work/company/microservice-name/`

### `--monday`, `--tuesday`, `--wednesday`, `--thursday`, `--friday`, `--saturday`, `--sunday` **(Optional)**
The maxiumn number of commits to create on that day of the week. It will generate a random amount of commits between zero and the number you supply. Highier the number, more commits are more likely. The default is `0` so if you don't set any days, no commits will be generated.

EG: `--monday 5` and `--tuesday 12`

### `--help`
Shows some help information.

EG: `--help`

### `--version`
Shows the version information.

EG: `--version`

## Ethics
This tool was created for me to learn and play with [Rust](https://www.rust-lang.org/). Is it cheating to make your GitHub look more active than it actually is? Yes, but whocares as the number of commits isn't an accurate measure of a persons skill. This isn't my idea and whilst searching for existing projects I found these.

- [fake-git-history](https://github.com/artiebits/fake-git-history) [JS]
- [Autopopulate your contribution graph](https://github.com/marketplace/actions/autopopulate-your-contribution-graph) [GitHub Action]

> Goodhart's law is an adage often stated as, "When a measure becomes a target, it ceases to be a good measure".

_Charles Goodhart, [Wikipedia](https://en.wikipedia.org/wiki/Goodhart%27s_law)_