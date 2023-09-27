# kun-bot
A simple Discord bot written purely in *rust* to provide randomly selected images from specified gallery(s).

```
Usage: kun-bot.exe [OPTIONS] <PATHS>... -- <ADMINS>...

Arguments:
  <PATHS>...
  <ADMINS>...

Options:
  -p, --prefix <PREFIX>  [default: s.]
  -t, --title <TITLE>    [default: kun-bot]
  -h, --help             Print help
  -V, --version          Print version
```
| ARG | Description |
| --- | - |
| PATHS | the list of directories containing images |
| ADMINS | the list of user IDs allowing whitelist privileges |
