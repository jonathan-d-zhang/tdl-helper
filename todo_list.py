import tdl_helper

def create_task():
    tdl_helper.create_task()

def print_tasks():
    tdl_helper.print_tasks()

def modify_task():
    tdl_helper.modify_task()

def delete_task():
    tdl_helper.delete_task()

def main():
    print("What would you like to do?")
    print("""\
1. Create a task
2. Look at the tasks
3. Modify a task
4. Delete a task
""")

    while True:
        try:
            choice = int(input())
            if not (1 <= choice <= 4):
                raise ValueError()

            break
        except ValueError:
            print("That wasn't a valid choice :(")


    if choice == 1:
        create_task()
    elif choice == 2:
        print_tasks()
    elif choice == 3:
        modify_task()
    elif choice == 4:
        delete_task()

main()
