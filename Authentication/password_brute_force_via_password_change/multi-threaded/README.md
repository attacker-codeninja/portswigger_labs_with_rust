# Hack Steps
1. Login with correct creds
2. Change username when requesting change password API
3. Repeat the process trying every password

# Run Script
1. Change the URL of the lab
2. Change the PATH for you passwords list
3. Change the the list splitter to \r\n instead of \n if you still a windows user
4. Start script
```
~$ cargo run
```

# Expected Output
```
[#] Brute forcing password of carlos..
[*] (30/100) mobilemail => Incorrect
[#] mom => Correct
[#] Password changed to: Hacked

✅ Finished in: 1 minutes
```
# Test Samples
This test is done using only 100 passwods. What about 10K passwords?
Or what about 100K passwords?

You can see the comparison I made with these numbers when solving the [Lab: Username enumeration via different responses](https://github.com/elqalawii/portswigger_labs_with_rust/tree/main/Authentication/username_enumeration_via_different_responses) to see the big difference in speed between Rust and Python and also between single-threaded and multi-threaded approaches in Rust.

