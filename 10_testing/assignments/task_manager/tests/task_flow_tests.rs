use task_manager::TaskManager;

#[test]
fn test_user_flow() {
    // New TaskManager
    let mut tm = TaskManager::new();

    // Add tasks
    let task_one_description = "First task";
    let next_id = tm.add_task(task_one_description.to_string());
    assert_eq!(next_id, 2);

    // Get and check tasks
    let task_one = tm.get_task(1).unwrap();
    assert_eq!(task_one.id, 1);
    assert_eq!(task_one.description, task_one_description);
    assert_eq!(task_one.completed, false);

    let task_two_description = "Second task";
    let next_id = tm.add_task(task_two_description.to_string());
    assert_eq!(next_id, 3);

    let task_two = tm.get_task(2).unwrap();
    assert_eq!(task_two.id, 2);
    assert_eq!(task_two.description, task_two_description);
    assert_eq!(task_two.completed, false);

    let task_three_description = "Third task";
    let next_id = tm.add_task(task_three_description.to_string());
    assert_eq!(next_id, 4);

    let task_three = tm.get_task(3).unwrap();
    assert_eq!(task_three.id, 3);
    assert_eq!(task_three.description, task_three_description);
    assert_eq!(task_three.completed, false);

    // List pending tasks
    let pending_tasks = tm.list_pending_tasks();
    assert_eq!(pending_tasks.len(), 3);
    assert_eq!(pending_tasks[0].id, 1);
    assert_eq!(pending_tasks[0].completed, false);

    assert_eq!(pending_tasks[1].id, 2);
    assert_eq!(pending_tasks[1].completed, false);

    assert_eq!(pending_tasks[2].id, 3);
    assert_eq!(pending_tasks[2].completed, false);

    // Mark one task as complete
    let complete_task = tm.mark_complete(3);
    assert!(complete_task.is_some());

    // List pending tasks again and verify the change
    let pending_tasks = tm.list_pending_tasks();
    assert_eq!(pending_tasks.len(), 2);
    assert_eq!(pending_tasks[0].id, 1);
    assert_eq!(pending_tasks[0].completed, false);

    assert_eq!(pending_tasks[1].id, 2);
    assert_eq!(pending_tasks[1].completed, false);

    // Remove an existing task
    let removed_task = tm.remove_task(1).unwrap();
    assert_eq!(removed_task.id, 1);
    assert_eq!(removed_task.description, task_one_description.to_string());
    assert_eq!(removed_task.completed, false);

    // Try to get the remove task
    let removed = tm.get_task(1);
    assert!(removed.is_none());

    // Remove a non existing task with id 999
    let removed_task = tm.remove_task(999);
    assert!(removed_task.is_none());

    // Add new task and check for the expected next_id = 5
    let next_id_5 = tm.add_task("important task".to_string());
    assert_eq!(next_id_5, 5);
}
