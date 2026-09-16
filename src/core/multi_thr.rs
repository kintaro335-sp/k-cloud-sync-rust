/*
 * k-cloud-frontend
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
*/

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::core::objects;

#[derive(Clone)]
pub struct TailTasks {
  pub tasks: Arc<Mutex<VecDeque<objects::FileObj>>>
}

impl TailTasks {
  pub fn new(new_tasks: Vec<objects::FileObj>) -> Self {
    Self {
      tasks: Arc::new(Mutex::new(VecDeque::from(new_tasks)))
    }
  }

  pub fn take_task(&self) -> Option<objects::FileObj> {
    let mut queue = self.tasks.lock().unwrap();
    queue.pop_front()
  }
}
