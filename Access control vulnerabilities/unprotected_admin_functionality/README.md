# Hack Steps

1. Fetch the /robots.txt file
2. Get the admin panel hidden path
3. Delete carlos

# Run Script

1. Change the URL of the lab
2. Start script

```
~$ cargo run
```

# Expected Output

```
1. Fetching /robots.txt file.. OK
2. Extracting the hidden path.. OK => /administrator-panel
3. Fetching the admin panel.. OK
4. Deleting carlos.. OK
[#] Check your browser, it should be marked now as solved
```