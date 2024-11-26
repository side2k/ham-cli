# Hamster CLI

Small pet project that syncs time records from [Hamster tracker](https://github.com/projecthamster/hamster) to [Everhour](https://app.everhour.com/) or Shtab(https://shtab.app). Most likely, it will not be of any use for anyone except myself.
It relies on the [everhour-simple-client](https://github.com/side2k/everhour-simple-client) and [shtab-simple-client](https://github.com/side2k/shtab-simple-client) crates, both of which are minimal-required-stuff API clients that I've created specifically for this purpose.

## Rationale

Locally, I'm using Hamster to track and plan my activity, while on my job I'm required to track my working time in Shtab(earlier - to Everhour). To avoid doing tracking double and also to learn some Rust - this project was created.

## How it works

### Everhour

For [adding time record entry in Everhour](https://everhour.docs.apiary.io/#reference/0/time-records/add-time), this data is required:

- Everhour API token: can be obtained one the [Everhour User Profile settings](https://app.everhour.com/#/account/profile) page. Can be specified via `--api-token` command line option or `EVERHOUR_API_TOKEN` environment variable.

- `task id` - in Hamster, for work task entries' description, I add links to Asana tasks in markdown format. Task ids are extracted from these links
- `user` - current user, obtained by [relevant API](https://everhour.docs.apiary.io/#reference/0/users/get-current-user) call
- `time` - time in seconds, calculated by the task duration, i.e. `end_time` - `start_time`. If end time is not defined, current time is used
- `date` - date of the task. Currently, tasks that span for more than 1 day, are not processed properly!


Basic example of running the sync:
```
ham-cli sync-eh Work
```
…will sync tasks of `Work` category for today. The command above assumes that:

### Shtab

This part is currently in a work-in-progress status.

It works mostly the same as Everhour sync, but with a few notes:

- obtaining Shtab API token: look for `AUTH` item in browser's local storage for `my.shtab.app` domain. For now.
- `activity id`(which is really team id - will be fixed soon) can be obtained from task URLs (for now only manual)
- for now, work time entries are not really "synced", they're merely added - so manual deletion is needed if you need to resync

## More information

- Hamster database is located in `$HOME/.local/share/hamster/hamster.db`
- Everhour API token is set in `EVERHOUR_API_TOKEN` env variable (see above)
- Shtab API token can be set to `HAMCLI_SHTAB_TOKEN` env variable
- Shtab activity id can be set to `HAMCLI_SHTAB_ACTIVITY` env variable
