# Hamster CLI

Small pet project that syncs time records from [Hamster tracker](https://github.com/projecthamster/hamster) to [Everhour](https://app.everhour.com/). Most likely, it will not be of any use except myself.
It relies on the [everhour-simple-client](https://github.com/side2k/everhour-simple-client) crate, that I've created specifically for this purpose.

## Rationale

Locally, I'm using Hamster to track and plan my activity, while on my job I'm required to track my working time in Everhour. To avoid doing tracking double and also to learn some Rust - this project was created.

## How it works

For [adding time record entry in Everhour](https://everhour.docs.apiary.io/#reference/0/time-records/add-time), this data is required:

- Everhour API token: can be obtained one the [Everhour User Profile settings](https://app.everhour.com/#/account/profile) page. Can be specified via `--api-token` command line option or `EVERHOUR_API_TOKEN` environment variable.

- `task id` - in Hamster, for work task entries' description, I add links to Asana tasks in markdown format. Task ids are extracted from these links
- `user` - current user, obtained by [relevant API](https://everhour.docs.apiary.io/#reference/0/users/get-current-user) call
- `time` - time in seconds, calculated by the task duration, i.e. `end_time` - `start_time`. If end time is not defined, current time is used
- `date` - date of the task. Currently, tasks that span for more than 1 day, are not processed properly!
