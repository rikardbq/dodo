## draft

![dodo](./mascot.webp)

### commands
- help
- new
    - \<name\>
- remove / rm
    - \<name\> or \<id\> of some kind
- list / ls (lists latest 20 or 50 tasks)
    - --filter -f (filter "dsl" goes here)
    - --all -a (lists all the tasks, may need to be paginated or something)
    - (maybe some flag to shortcut common use cases, i.e list all high prio tasks, kind of like `dodo ls --high`, otherwise `dodo ls --filter p:high`)

### structure
- todos are local to the folder you are standing in when executing commands
- todo named root folder of a todo task is **YYYY-MM-DD** format
    - inside is a file per task
        - formatted according to:
        - **id?** (perhaps a checksum derived from the task contents)
        - **name**
        - **description** (optional)
        - **keywords** (optional)
        - **priority** (optional)
    - searchable on date, name, description, keywords, priority
        - filters will be composable
        - i.e (((this and that) and not thing) or other)
    - done tasks are in done folder inside the **YYYY-MM-DD** folder
