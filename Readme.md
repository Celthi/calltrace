# Call Trace Assistant

(Work-in-progress)

A service to structure your call traces (call stacks)

Call traces (stacks) are valuable assets for large programs. In a large program, there are many ways to reach the same function (line). Without contexts, we know little. Those call traces (stacks) are the context that we are looking for.


# Support Functionalities

1. Fold call stacks
2. Capture the extra call stacks comparing to a base point (either a past time or a different env)
3. Filter the call stacks and only keep those you're interested.

# TODO

- [ ] Persist call stacks into database.

- [ ] Query persisted call stacks by keyword.

- [ ] Service to expose those APIs
