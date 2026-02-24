#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub completed: bool,
}

pub struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: Vec::<Task>::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String) -> u32 {
        let current_id = self.next_id;

        self.tasks.push(Task {
            id: current_id,
            description,
            completed: false,
        });

        self.next_id = self.next_id + 1;
        self.next_id
    }

    pub fn mark_complete(&mut self, task_id: u32) -> Option<()> {
        if let Some(result) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            result.completed = true;
            return Some(());
        }

        None
    }

    pub fn get_task(&self, task_id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == task_id)
    }

    pub fn list_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| !t.completed).collect()
    }

    pub fn remove_task(&mut self, taks_id: u32) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == taks_id) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taskmanager_new() {
        let tm = TaskManager::new();
        assert!(tm.tasks.is_empty(), "Tasks vector should be empty");
        assert_eq!(tm.next_id, 1, "Next id should be 1");
    }

    #[test]
    fn test_add_task() {
        let mut tm = TaskManager::new();
        let task_description = "A new task here";
        let next_id = tm.add_task(task_description.to_string());

        assert_eq!(next_id, 2);
        assert_eq!(tm.tasks.len(), 1);
        assert_eq!(tm.tasks[0].id, 1);
        assert_eq!(tm.tasks[0].description, task_description);
        assert_eq!(tm.tasks[0].completed, false);
    }

    #[test]
    fn test_mark_complete_existing_task() {
        let mut tm = TaskManager::new();
        let task_description = "A new task here";
        let next_id = tm.add_task(task_description.to_string());

        assert_eq!(next_id, 2);
        assert_eq!(tm.tasks.len(), 1);
        assert_eq!(tm.tasks[0].id, 1);
        assert_eq!(tm.tasks[0].description, task_description);
        assert_eq!(tm.tasks[0].completed, false);

        tm.mark_complete(1);
        assert_eq!(tm.tasks[0].completed, true);
    }

    #[test]
    fn test_mark_complete_non_existing_task() {
        let mut tm = TaskManager::new();
        let task_description = "A new task here";
        let next_id = tm.add_task(task_description.to_string());

        assert_eq!(next_id, 2);
        assert_eq!(tm.tasks.len(), 1);
        assert_eq!(tm.tasks[0].id, 1);
        assert_eq!(tm.tasks[0].description, task_description);
        assert_eq!(tm.tasks[0].completed, false);

        assert_eq!(tm.mark_complete(2), None);
        assert_eq!(tm.tasks[0].completed, false);
    }

    #[test]
    fn test_mark_complete_a_completed_task() {
        let mut tm = TaskManager::new();
        let task_description = "A new task here";
        let next_id = tm.add_task(task_description.to_string());

        assert_eq!(next_id, 2);
        assert_eq!(tm.tasks.len(), 1);
        assert_eq!(tm.tasks[0].id, 1);
        assert_eq!(tm.tasks[0].description, task_description);
        assert_eq!(tm.tasks[0].completed, false);

        assert_eq!(tm.mark_complete(1), Some(()));
        assert_eq!(tm.tasks[0].completed, true);
        assert_eq!(tm.mark_complete(1), Some(()));
        assert_eq!(tm.tasks[0].completed, true);
    }

    #[test]
    fn test_get_task_existing_task() {
        let mut tm = TaskManager::new();
        let task_description = "A new task here";
        let next_id = tm.add_task(task_description.to_string());
        assert_eq!(next_id, 2);

        let task = tm.get_task(1).unwrap();
        assert_eq!(task.id, 1);
        assert_eq!(task.description, task_description);
        assert_eq!(task.completed, false);
    }

    #[test]
    fn test_get_task_non_existing_task() {
        let mut tm = TaskManager::new();
        let task_description = "A new task here";
        let next_id = tm.add_task(task_description.to_string());
        assert_eq!(next_id, 2);

        let task = tm.get_task(2);
        assert!(task.is_none());
    }

    #[test]
    fn test_list_pending_tasks_empty() {
        let tm = TaskManager::new();
        let pending = tm.list_pending_tasks();
        assert_eq!(pending.len(), 0);
    }

    #[test]
    fn test_list_pending_tasks_some_completed_some_pending() {
        let mut tm = TaskManager::new();
        let task_description = "First task";
        let next_id_1 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_1, 2);

        let task_description = "Second task";
        let next_id_2 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_2, 3);

        let task_description = "Third task";
        let next_id_3 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_3, 4);

        tm.mark_complete(1);

        let pending = tm.list_pending_tasks();
        assert_eq!(pending.len(), 2);
        assert_eq!(pending[0].id, 2);
        assert_eq!(pending[1].id, 3);
    }

    #[test]
    fn test_list_pending_tasks_all_pending() {
        let mut tm = TaskManager::new();
        let task_description = "First task";
        let next_id_1 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_1, 2);

        let task_description = "Second task";
        let next_id_2 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_2, 3);

        let task_description = "Third task";
        let next_id_3 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_3, 4);

        let pending = tm.list_pending_tasks();
        assert_eq!(pending.len(), 3);

        assert_eq!(pending[0].id, 1);
        assert_eq!(pending[0].completed, false);

        assert_eq!(pending[1].id, 2);
        assert_eq!(pending[1].completed, false);

        assert_eq!(pending[2].id, 3);
        assert_eq!(pending[2].completed, false);
    }

    #[test]
    fn test_list_pending_tasks_all_completed() {
        let mut tm = TaskManager::new();
        let task_description = "First task";
        let next_id_1 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_1, 2);

        let task_description = "Second task";
        let next_id_2 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_2, 3);

        let task_description = "Third task";
        let next_id_3 = tm.add_task(task_description.to_string());
        assert_eq!(next_id_3, 4);

        tm.mark_complete(1);
        tm.mark_complete(2);
        tm.mark_complete(3);

        let pending = tm.list_pending_tasks();
        assert_eq!(pending.len(), 0);
    }

    #[test]
    fn test_remove_task_existing_task() {
        let mut tm = TaskManager::new();
        let task_description_1 = "First task";
        let next_id_1 = tm.add_task(task_description_1.to_string());
        assert_eq!(next_id_1, 2);

        let task_description_2 = "Second task";
        let next_id_2 = tm.add_task(task_description_2.to_string());
        assert_eq!(next_id_2, 3);

        let removed_task = tm.remove_task(1).unwrap();
        assert_eq!(removed_task.id, 1);
        assert_eq!(removed_task.description, task_description_1);
        assert_eq!(removed_task.completed, false);
    }

    #[test]
    fn test_remove_task_non_existent_task() {
        let mut tm = TaskManager::new();
        let task_description_1 = "First task";
        let next_id_1 = tm.add_task(task_description_1.to_string());
        assert_eq!(next_id_1, 2);

        let removed_task = tm.remove_task(2);
        assert!(removed_task.is_none());
    }

    #[test]
    fn test_remove_task_ensure_next_id_not_affected() {
        let mut tm = TaskManager::new();
        let task_description_1 = "First task";
        let next_id_1 = tm.add_task(task_description_1.to_string());
        assert_eq!(next_id_1, 2);

        let removed_task = tm.remove_task(1).unwrap();
        assert_eq!(removed_task.id, 1);
        assert_eq!(removed_task.description, task_description_1);
        assert_eq!(removed_task.completed, false);

        let task_description_2 = "Second task";
        let next_id_2 = tm.add_task(task_description_2.to_string());
        assert_eq!(next_id_2, 3);
    }
}
