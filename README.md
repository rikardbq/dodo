## draft

![dodo](./mascot.webp)

### ideas for the commands
- **new** (name#description#keywords)
    - `dodo new "update packages#this is a description#medium,risky,testing"`
    - no description, `dodo new "update packages##medium,risky,testing"`
    - no keywords, `dodo new "update packages#this is a description"`
    - no description or keywords `dodo new "update packages"`
- **done** (name)
    - `dodo done "update packages"`
        - this moves the task from its date folder to the **done** folder inside of that

### commands
- help
- new
    - \<name\>
- remove / rm
    - \<name\> or \<id\> of some kind
- list / ls (lists latest 20 or 50 tasks)
    - --filter -f (filter "dsl" goes here)
    - --all -a (lists all the tasks, may need to be paginated or something)
    - --done -d (list all done tasks)
    - (maybe some flag to shortcut common use cases, i.e list all high prio tasks, kind of like `dodo ls --high`, otherwise `dodo ls --filter p:high`)
- done
    - \<name\> or \<id\>

### structure
- todos are local to the folder you are standing in when executing commands
- todo named root folder of a todo task is **YYYYMMDD** format
    - inside is a file per task
        - formatted according to:
        - **id?** (perhaps a checksum derived from the task contents)
        - **name**
        - **description** (optional)
        - **priority** (optional)
        - **keywords** (optional)
    - searchable on date, name, description, keywords, priority
        - filters will be composable
        - i.e (((this and that) and not thing) or other)
    - done tasks are in done folder inside the **YYYYMMDD** folder
    - **example folder structure**
        - dodos (gives the todos a root path to not collide with any other folders which may use the same naming)
            - 20260623 (folder)
                - update_readme (name of file)
                - ```
                    update_readme
                    #####
                    update the readme with some examples
                    #####
                    low,readme,examples
                - done (folder)
                    - create_repo (name of done file)
