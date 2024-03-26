This utility will print you the amount of RAM being used in your system in total, as well as sum all the RAM used by processes that match a working path defined in the .env.

For example, if your env is:

```
PROCESS_REGEX=multitest.*
```

It will match files in /home/christianlarrabure/develop/multitest-resources/ or anywhere that has that word.
