
use core::sync::atomic::{AtomicUsize, Ordering};

pub const MAX_TASKS: usize = 16;
const STACK_SIZE: usize = 4096;

#[derive(Copy, Clone)]
pub enum TaskState {
    Available,
    Running,
    Ready,
    Blocked,
}

pub struct Task {
    pub id: usize,
    pub stack: [u8; STACK_SIZE],
    pub stack_ptr: usize,
    pub state: TaskState,
}

pub struct Scheduler {
    tasks: [Task; MAX_TASKS],
    current_task: AtomicUsize,
}

impl Scheduler {
    pub const fn new() -> Self {
        const EMPTY_TASK: Task = Task {
            id: 0,
            stack: [0; STACK_SIZE],
            stack_ptr: 0,
            state: TaskState::Available,
        };
        
        Scheduler {
            tasks: [EMPTY_TASK; MAX_TASKS],
            current_task: AtomicUsize::new(0),
        }
    }

    pub fn create_task(&mut self, entry: fn() -> !) -> Option<usize> {
        // Find available slot
        let slot = self.tasks.iter()
            .position(|task| matches!(task.state, TaskState::Available))?;

        // Initialize task stack
        let stack_top = self.tasks[slot].stack.as_ptr() as usize + STACK_SIZE;
        
        // Create initial stack frame
        unsafe {
            let frame = (stack_top - core::mem::size_of::<usize>()) as *mut usize;
            *frame = entry as usize;  // Entry point
        }

        self.tasks[slot].stack_ptr = stack_top - 32 * 8;  // Space for registers
        self.tasks[slot].state = TaskState::Ready;
        self.tasks[slot].id = slot;

        Some(slot)
    }

    pub fn schedule(&self) -> Option<&Task> {
        let current = self.current_task.load(Ordering::Relaxed);
        let next = self.tasks.iter()
            .cycle()
            .skip(current + 1)
            .take(MAX_TASKS)
            .find(|task| matches!(task.state, TaskState::Ready))?;

        self.current_task.store(next.id, Ordering::Relaxed);
        Some(next)
    }
}

#[no_mangle]
pub static mut SCHEDULER: Scheduler = Scheduler::new();
